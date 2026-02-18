hi! my name is attila.
i am a phd candidate at the institute of mathematics of tu berlin funded by the german research foundation within subproject [b03](https://gepris.dfg.de/gepris/projekt/259091447) of the [trr154](https://gepris.dfg.de/gepris/projekt/239904186?language=en).
take a look below to learn more about my research.

<br>

## research interests
- nonlinear energy-based systems (port-hamiltonian, passive, dissipative)
- structured space and time discretization and model reduction (petrov-galerkin, discrete gradient)
- structured optimal control and turnpike properties
- structured feedback control and state estimation

<br>


## papers

| year | title                                                        | authors          | note      |
| ---- | ------------------------------------------------------------ | ---------------- | --------- |
2026 | [a discrete gradient scheme for preserving qsr-dissipativity](https://arxiv.org/abs/2502.04987) | a.karsai, p.schulze | preprint |
2025 | [nonlinear systems and passivity: feedback control, model reduction, and time discretization](https://arxiv.org/abs/2502.04987) | t.breiten, a.karsai | preprint |
2025 | [structure-preserving discretization and model reduction for energy-based models](https://arxiv.org/abs/2507.21552) | r.altmann, a.karsai, p.schulze | preprint |
2025 | [passivity encoding representations of nonlinear systems](https://doi.org/10.1109/TAC.2025.3576535) | a.karsai, t.breiten, j.ramme, p.schulze | published <br><small style="color:green;">open access</small> |
2025 | [energy-consistent petrov-galerkin time discretization of port-hamiltonian systems](https://doi.org/10.5802/smai-jcm.127) | j.giesselmann, a.karsai, t.tscherpel  | published <br><small style="color:green;">open access</small> |
2024 | [manifold turnpikes of nonlinear port-hamiltonian descriptor systems under minimal energy supply](https://doi.org/10.1007/s00498-024-00384-7) | a.karsai             | published <br><small style="color:green;">open access</small> |
2023 | [structure-preserving \\(\mathcal{H}_{\infty}\\) control for port-hamiltonian systems](https://doi.org/10.1016/j.sysconle.2023.105493) | t.breiten, a.karsai | published |


<br>

## cv
- **may 2022 - today:** phd candidate in mathematics (expected complection march 2026)
- **october 2016 - february 2022:** bachelors & masters in mathematics
    - master thesis: `structure-preserving control of port-hamiltonian systems`. supervised by t. breiten
    - bachelor thesis: `computation of the distance to instability for large systems`. supervised by v. mehrmann
- **awards:**
    - [GAMM junior](https://www.gamm-juniors.de/) (2026 - 2028): awarded 3-year GAMM junior status for exceptional thesis in applied mathematics
    - [best phd talk @ 16th elgersburg workshop](https://www.tu-ilmenau.de/systpde/elgersburg-workshop) (2024): selected by senior researchers for clarity of presentation and research impact
    - [verein deutscher ingenieure](https://www.vdi.de/) (2022): received recognition for graduates with outstanding exam results
- **teaching experience:**
    - numerical mathematics 1 (assistant & tutor): conceptualized and corrected assignments and exams, coordinated tutors, held exercise and tutorial sessions
    - linear algebra 1+2 (tutor): corrected assignments and exams, held tutorial sessions


feel free to reach out for a complete cv

<br>

## research

my research focuses on **nonlinear dynamical systems** consisting of ordinary and partial differential-algebraic equations.
since dynamical systems are often rooted in physical processes, they share physical properties such as **conservation or dissipation of energy** or certain symmetries with their real-world counterparts.
during my phd, my main interest were systems with such energy properties.

key questions are:
- what are suitable model classes to describe nonlinear energy-based phenomena?
- how can dynamical systems be discretized in the temporal and spatial variables while preserving the energy-based viewpoint?
- what is the behavior of optimal controls when the energy structure is taken into account in the optimization objective, and how can we construct controllers that are interpretable as energy-based systems?


### time discretization

to illustrate the importance of these questions, below the energy of a nonlinear passive system is shown after a time-discrete solution was obtained with 
- the implicit midpoint method (generally *not* structure-preserving for nonlinear systems), and 
- a discrete gradient method suitable for systems dissipative w.r.t. a quadratic supply rate.

<div id="simulation-container">
    <div hidden id="status" class="loading">loading wasm module...</div>

    <div class="controls">
        
        <div class="control-group">
            <label for="delta-t-slider">step size:</label>
            <input type="range" id="delta-t-slider" min="0.01" max="0.25" step="0.01" value="0.25">
            <label for="delta-t-slider">
                <span id="delta-t-value" class="value-display">
                    0.25
                </span>
            </label>
        </div>
    </div>

    <div id="plot"></div>

    <div style="text-align: center; font-size: 0.85em;">
        made with 
        <a href="https://www.rust-lang.org/" target="_blank">Rust</a>, 
        <a href="https://webassembly.org/" target="_blank">WebAssembly</a> 
        and 
        <a href="https://plotly.com/" target="_blank">plotly</a>
    </div>
</div><br>

for the control input $$u=0$$, the energy should not increase.
nevertheless, we see that for larger choices of the time step size, an increase of the energy is possible for the implicit midpoint method.
the discrete gradient method does not exhibit this behavior.





<script src="https://cdn.plot.ly/plotly-2.27.0.min.js"></script>
<script type="module" src="{{ '/assets/js/simulation.js' | relative_url }}"></script>




