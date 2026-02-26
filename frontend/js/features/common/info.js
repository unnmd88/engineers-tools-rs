// features/common/info.js

window.renderCommonInfo = async function() {
    const content = document.getElementById('feature-content');
    
    try {
        const data = await api.get('/info');
        
        content.innerHTML = `
            <h2>ℹ️ System Information</h2>
            
            <div style="background: white; padding: 20px; border-radius: 8px; margin-top: 20px;">
                <pre style="background: #f5f5f5; padding: 15px; border-radius: 6px;">${JSON.stringify(data, null, 2)}</pre>
            </div>
        `;
    } catch (error) {
        content.innerHTML = `<div class="status error">Ошибка: ${error.message}</div>`;
    }
};