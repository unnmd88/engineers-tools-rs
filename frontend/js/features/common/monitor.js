window.renderCommonMonitor = async function () {
    console.log("renderMonitor");
    const content = document.getElementById('feature-content');

    content.innerHTML = `
        <h2>📡 Monitor</h2>
        
        <label>IP:</label>
        <input id="ip" value="192.168.1.1"/><br/><br/>
        
        <label>Interval (sec):</label>
        <input id="interval" value="1"/><br/><br/>
        
        <button id="start">Start</button>
        <button id="stop">Stop</button>
        
        <pre id="output"></pre>
    `;

    let ws;

    document.getElementById('start').onclick = () => {
        // ws = new WebSocket(`ws://${window.APP_CONFIG.host}:${window.APP_CONFIG.port}/ws/monitor`);
        ws = new WebSocket(`ws://${window.APP_CONFIG.host}:${window.APP_CONFIG.port}/api/v1/common/ws/monitor`);

        ws.onopen = () => {
            const config = {
                ip: document.getElementById('ip').value,
                interval: Number(document.getElementById('interval').value)
            };

            ws.send(JSON.stringify(config));
        };

        ws.onmessage = (e) => {
            document.getElementById('output').textContent = e.data;
        };

        ws.onclose = () => {
            console.log("WS closed");
        };
    };

    document.getElementById('stop').onclick = () => {
        if (ws) {
            ws.close();
        }
    };
};