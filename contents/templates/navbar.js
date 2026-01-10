(() => {
  let navbar_id = "";
  if (window.location.pathname === "/") {
    navbar_id = "home";
  } else if (window.location.pathname.startsWith("/posts")) {
    navbar_id = "posts";
  } else if (window.location.pathname.startsWith("/contact")) {
    navbar_id = "contact";
  }
  if (navbar_id !== "") {
    document.querySelector("#" + navbar_id)?.classList.add("active");
  }
})();
