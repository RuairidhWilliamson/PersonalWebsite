const toggle = document.querySelector("#theme-toggle");

const getTheme = () => {
  const theme = window.localStorage.getItem("theme");
  if (theme === "dark" || theme === "light") {
    return theme;
  }
  if (!window.matchMedia) {
    // If we can't get os color scheme default to light
    return "light";
  }
  // Use os prefered color scheme
  if (window.matchMedia('(prefers-color-scheme: dark)').matches) {
    return "dark";
  }
  if (window.matchMedia('(prefers-color-scheme: light)').matches) {
    return "light";
  }
};

const updateTheme = () => {
  const theme = getTheme();
  if (theme === "dark") {
    document.body.classList.add("theme-dark");
    document.body.classList.remove("theme-light");
  } else if (theme === "light") {
    document.body.classList.remove("theme-dark");
    document.body.classList.add("theme-light");
  } else {
    console.log("unknown theme:", theme);
  }
};

toggle.addEventListener("click", () => {
  const theme = getTheme();
  window.localStorage.setItem("theme", theme === "dark" ? "light" : "dark");
  updateTheme();
});
updateTheme();


window.matchMedia('(prefers-color-scheme: dark)').addEventListener("change", () => {
  updateTheme();
});
