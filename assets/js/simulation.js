// simulation.js - merged frontend and backend

// ===== BACKEND (WASM interface and simulation logic) =====

let wasmModule = null;
let wasmReady = false;
let traceRef = null;

// Simulation constants
const T_FINAL = 10.0;
const DELTA_T_REF = 0.001;

// Initialize WASM module
export async function initSimulation() {
    try {
        const { default: init, compute_energy } = await import('./pkg/dg_simulator.js');
        await init();
        wasmModule = { compute_energy };
        wasmReady = true;
        console.log('WebAssembly module loaded successfully');
        return { success: true };
    } catch (err) {
        console.error('Failed to load WASM module:', err);
        return { success: false, error: err };
    }
}

// Check if module is ready
export function isReady() {
    return wasmReady;
}

// Get current parameters from UI elements
export function getParameters() {
    // const systemKind = document.getElementById('system-select')?.value || 'toda';
    const systemKind = 'toda';
    const deltaT = parseFloat(document.getElementById('delta-t-slider')?.value || 0.25);
    const tFinal = T_FINAL;
    const nt = Math.floor(tFinal / deltaT);
    return { systemKind, deltaT, tFinal, nt };
}

// Compute reference solution
export function computeReferenceSolution(systemKind = 'toda') {
    if (!wasmReady || !wasmModule) {
        throw new Error('WASM module not initialized');
    }

    const ntRef = Math.floor(T_FINAL / DELTA_T_REF);
    const ttRef = Array.from({length: ntRef}, (_, i) => i * DELTA_T_REF);
    const start = performance.now();
    const hhRef = wasmModule.compute_energy(systemKind, T_FINAL, DELTA_T_REF, "implicit midpoint");
    const elapsed = performance.now() - start;
    
    traceRef = {
        x: ttRef,
        y: hhRef,
        mode: 'lines',
        name: 'reference',
        line: {
            color: 'rgba(0, 0, 0, 0.2)',
            width: 3,
            dash: 'dot'
        }
    };
    
    console.log(`reference solution computed in ${elapsed.toFixed(1)}ms`);
    return { trace: traceRef, elapsed };
}

// Run implicit midpoint simulation
export function runImplicitMidpoint(systemKind, deltaT, tFinal = T_FINAL) {
    if (!wasmReady || !wasmModule) {
        throw new Error('WASM module not initialized');
    }

    const start = performance.now();
    const nt = Math.floor(tFinal / deltaT);
    const tt = Array.from({length: nt}, (_, i) => i * deltaT);
    const hhIm = wasmModule.compute_energy(systemKind, tFinal, deltaT, "implicit midpoint");
    const elapsed = performance.now() - start;
    
    const trace = {
        x: tt,
        y: hhIm,
        mode: 'lines',
        name: 'implicit midpoint',
        line: {
            color: 'rgba(214, 39, 40, 0.6)',
            width: 4
        }
    };
    
    return { trace, elapsed, time: tt, hamiltonian: hhIm };
}

// Run discrete gradient simulation
export function runDiscreteGradient(systemKind, deltaT, tFinal = T_FINAL) {
    if (!wasmReady || !wasmModule) {
        throw new Error('WASM module not initialized');
    }

    const start = performance.now();
    const nt = Math.floor(tFinal / deltaT);
    const tt = Array.from({length: nt}, (_, i) => i * deltaT);
    const hhDg = wasmModule.compute_energy(systemKind, tFinal, deltaT, "discrete gradient");
    const elapsed = performance.now() - start;
    
    const trace = {
        x: tt,
        y: hhDg,
        mode: 'lines',
        name: 'discrete gradient',
        line: {
            color: 'rgba(31, 119, 180, 0.6)',
            width: 4
        }
    };
    
    return { trace, elapsed, time: tt, hamiltonian: hhDg };
}

// Run both simulations
export function runBothSimulations(systemKind, deltaT, tFinal = T_FINAL) {
    if (!wasmReady || !wasmModule) {
        throw new Error('WASM module not initialized');
    }

    const start = performance.now();
    const imResult = runImplicitMidpoint(systemKind, deltaT, tFinal);
    const dgResult = runDiscreteGradient(systemKind, deltaT, tFinal);
    const totalElapsed = performance.now() - start;
    
    return {
        im: imResult,
        dg: dgResult,
        totalElapsed
    };
}

// Get all traces for plotting (reference + current simulations)
export function getAllTraces(systemKind, deltaT) {
    if (!traceRef) {
        computeReferenceSolution(systemKind);
    }

    const imResult = runImplicitMidpoint(systemKind, deltaT);
    const dgResult = runDiscreteGradient(systemKind, deltaT);
    
    return {
        traces: [traceRef, dgResult.trace, imResult.trace],
        elapsed: {
            im: imResult.elapsed,
            dg: dgResult.elapsed
        }
    };
}

// Get reference trace only
export function getReferenceTrace() {
    return traceRef;
}

// ===== FRONTEND (UI and plotting logic) =====

const textColor = getComputedStyle(document.documentElement).getPropertyValue('--clr-text').trim();
const gridColor = textColor.replace('hsl(', 'hsla(').replace(')', ', 0.3)');
const bgColor = getComputedStyle(document.documentElement).getPropertyValue('--clr-bg').trim();

let referenceComputed = false;

// Initialize WASM and UI
async function init() {
    const result = await initSimulation();
    const statusEl = document.getElementById('status');
    
    if (result.success) {
        statusEl.textContent = 'ready! adjust time step as wanted';
        statusEl.className = 'ready';
        computeReference();
        updatePlot();
    } else {
        statusEl.textContent = 'error loading module: ' + result.error;
        statusEl.className = 'error';
    }
}

// Compute reference solution once
function computeReference() {
    if (referenceComputed || !isReady()) return;
    
    try {
        // const systemKind = 'document.getElementById('system-select').value';
        const systemKind = 'toda';
        const result = computeReferenceSolution(systemKind);
        referenceComputed = true;
    } catch (err) {
        console.error('error computing reference:', err);
    }
}

// Update plot with current parameters
function updatePlot() {
    if (!isReady() || !referenceComputed) return;

    const statusEl = document.getElementById('status');
    
    try {
        // const systemKind = document.getElementById('system-select').value;
        const systemKind = 'toda';
        const deltaT = parseFloat(document.getElementById('delta-t-slider').value);
        
        // Update display
        document.getElementById('delta-t-value').textContent = deltaT.toFixed(2);
        
        statusEl.textContent = 'computing...';
        statusEl.className = 'loading';

        // Small delay to allow UI update
        setTimeout(() => {
            try {
                const result = getAllTraces(systemKind, deltaT);
                
                const layout = {
                    xaxis: {
                        title: 'time',
                        fixedrange: true,
                        gridcolor: gridColor,
                        color: textColor,
                    },
                    yaxis: {
                        title: 'energy',
                        fixedrange: true,
                        gridcolor: gridColor,
                        color: textColor,
                    },
                    legend: {
                        x: 0.98,
                        y: 0.98,
                        xanchor: 'right',
                        yanchor: 'top',
                        traceorder: 'reversed',
                    },
                    margin: {
                        l: 60,
                        r: 30,
                        t: 30,
                        b: 50
                    },
                    paper_bgcolor: 'rgba(0,0,0,0)',
                    plot_bgcolor: 'rgba(0,0,0,0)',
                    autosize: true,
                };

                const config = {
                    displayModeBar: false,
                    responsive: true,
                };

                Plotly.newPlot('plot', result.traces, layout, config);
                
                statusEl.textContent = `implicit midpoint: ${result.elapsed.im.toFixed(1)}ms, discrete gradient: ${result.elapsed.dg.toFixed(1)}ms`;
                console.log(`implicit midpoint and discrete gradient solutions computed in ${result.elapsed.im.toFixed(1)}ms and ${result.elapsed.dg.toFixed(1)}ms`);
                statusEl.className = 'ready';
            } catch (err) {
                statusEl.textContent = 'Error: ' + err.message;
                statusEl.className = 'error';
                console.error(err);
            }
        }, 10);
    } catch (err) {
        statusEl.textContent = 'Error: ' + err.message;
        statusEl.className = 'error';
        console.error(err);
    }
}

// Event listeners
// document.getElementById('system-select').addEventListener('change', () => {
//     referenceComputed = false;
//     computeReference();
//     updatePlot();
// });

document.getElementById('delta-t-slider').addEventListener('input', updatePlot);

// Initialize on load
init();
