(() => {
  const eventSource = new EventSource("/hr");
  const hotReloadId = "%HOTRELOADID%";
  eventSource.onmessage = (event) => {
    console.log(event);
    if (event.data !== hotReloadId) {
      console.log(`Reloading ${hotReloadId} to ${event.data}`);
      window.location.reload();
    } else {
      console.log(`Not reloading id matches ${hotReloadId}`);
    }
  };
})();
