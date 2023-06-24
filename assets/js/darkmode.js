// darkmode.js

document.addEventListener("DOMContentLoaded", function() {

  // Get the container elements
  var feedback = document.getElementById("feedback");
  var turnpike = document.getElementById("turnpike");
  var example = document.getElementById("example");
  
  // Function to handle the dark mode preference change
  function handleDarkModeChange(e) {
    // Check if dark mode is enabled
    if (e.matches) {
      // Dark mode enabled
      feedback.classList.add("darkmode");
      turnpike.classList.add("darkmode");
      example.classList.add("darkmode");
    } else {
      // Dark mode disabled
      feedback.classList.remove("darkmode");
      turnpike.classList.remove("darkmode");
      example.classList.remove("darkmode");
    }
  }
  
  // Listen for changes to the user's preferred color scheme
  var darkModeQuery = window.matchMedia("(prefers-color-scheme: dark)");
  darkModeQuery.addListener(handleDarkModeChange);

  // Initial check to apply dark mode if it's enabled
  handleDarkModeChange(darkModeQuery);
  
});
