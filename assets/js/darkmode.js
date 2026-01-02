// darkmode.js

document.addEventListener("DOMContentLoaded", function() {

  // Get the container elements
  var simulation = document.getElementById("simulation-container");
  
  // Function to handle the dark mode preference change
  function handleDarkModeChange(e) {
    // Check if dark mode is enabled
    if (e.matches) {
      // Dark mode enabled
      simulation.classList.add("darkmode");
    } else {
      // Dark mode disabled
      simulation.classList.remove("darkmode");
    }
  }
  
  // Listen for changes to the user's preferred color scheme
  var darkModeQuery = window.matchMedia("(prefers-color-scheme: dark)");
  darkModeQuery.addListener(handleDarkModeChange);

  // Initial check to apply dark mode if it's enabled
  handleDarkModeChange(darkModeQuery);
  
});
