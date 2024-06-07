function(event) {
  function handle_event(symbol, value) {
    switch (symbol) {
      case "limiter":
        const limiter = event.icon.find("[mod-port-symbol=limiter]");
        if(value == 1) {
          limiter.addClass("on");
        } else {
          limiter.removeClass("on");
        }
        break;
      default:
        break;
    }
  }

  if (event.type == 'start') {
    const ports = event.ports;
    for (const p in ports) {
      handle_event(ports[p].symbol, ports[p].value);
    }
  }
  else if (event.type == 'change') {  
    handle_event(event.symbol, event.value);
  }
}