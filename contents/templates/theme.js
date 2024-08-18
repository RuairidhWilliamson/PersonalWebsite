const root = document.documentElement;
const themePrefKey = "theme_pref";

const themePreference = window.localStorage.getItem(themePrefKey);
if (themePreference === "theme-dark") {
  root.classList.add("theme-dark");
} else if (themePreference === "theme-light") {
  root.classList.add("theme-light");
}

function toggleTheme() {
  if (root.classList.contains("theme-dark")) {
    root.classList.remove("theme-dark");
    setTheme("theme-light");
  } else if (root.classList.contains("theme-light")) {
    root.classList.remove("theme-light");
    setTheme("theme-dark");
  } else if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches) {
    setTheme("theme-light");
  } else if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches) {
    setTheme("theme-dark");
  } else {
    setTheme("theme-dark");
  }
}

function setTheme(theme) {
  root.classList.add(theme);
  window.localStorage.setItem(themePrefKey, theme);
}

function keyboardSelect(event, f) {
  switch (event.which) {
    case 13: // KEY_ENTER
    case 32: // KEY_SPACE
      f();
      break;
    default:
  }
}
