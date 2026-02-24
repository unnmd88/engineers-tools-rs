// Глобальная навигация
window.showPage = function(pageName) {
    // Скрываем все страницы
    document.querySelectorAll('.page').forEach(page => {
        page.classList.remove('active');
    });
    
    // Показываем выбранную
    const activePage = document.getElementById(pageName + '-page');
    activePage.classList.add('active');
    
    // Вызываем функцию рендера страницы, если она есть
    if (window['render' + pageName]) {
        window['render' + pageName]();
    }
};

// Базовый URL из конфига (будет вставлен бэкендом)
const API_BASE = window.APP_CONFIG 
    ? `http://${window.APP_CONFIG.host}:${window.APP_CONFIG.port}`
    : '';

// Универсальная функция для API запросов
window.apiRequest = async function(endpoint, options = {}) {
    const url = API_BASE ? `${API_BASE}${endpoint}` : endpoint;
    
    try {
        const response = await fetch(url, {
            ...options,
            headers: {
                'Content-Type': 'application/json',
                ...options.headers
            }
        });
        
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        
        return await response.json();
    } catch (error) {
        console.error('API Error:', error);
        throw error;
    }
};

// При загрузке показываем страницу info
document.addEventListener('DOMContentLoaded', () => {
    showPage('info');
});