// Live reload functionality
(function() {
    const ws = new WebSocket(`ws://${window.location.host}/livereload`);
    
    ws.onmessage = function(event) {
        if (event.data === 'reload') {
            window.location.reload();
        }
    };
    
    ws.onclose = function() {
        // Try to reconnect after 1 second
        setTimeout(function() {
            window.location.reload();
        }, 1000);
    };
})();