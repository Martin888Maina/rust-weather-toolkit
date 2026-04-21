const cityInput = document.getElementById('city-input');
const searchBtn = document.getElementById('search-btn');
const errorMsg = document.getElementById('error-msg');
const loadingEl = document.getElementById('loading');
const weatherCard = document.getElementById('weather-card');

cityInput.addEventListener('keydown', function(e) {
    if (e.key === 'Enter') fetchWeather();
});

async function fetchWeather() {
    const city = cityInput.value.trim();

    if (!city) {
        showError('Please enter a city name.');
        return;
    }

    setLoading(true);
    clearError();
    weatherCard.classList.add('hidden');

    try {
        const res = await fetch('/api/weather?city=' + encodeURIComponent(city));
        const data = await res.json();

        if (!res.ok) {
            showError(data.error || 'Something went wrong. Please try again.');
            return;
        }

        renderWeather(data);
    } catch (err) {
        showError('Network error. Please check your connection and try again.');
    } finally {
        setLoading(false);
    }
}

function renderWeather(data) {
    document.getElementById('city-name').textContent = data.city;
    document.getElementById('country-badge').textContent = data.country;
    document.getElementById('description').textContent = data.description;
    document.getElementById('temperature').textContent = data.temperature.toFixed(1) + ' °C';
    document.getElementById('feels-like').textContent = data.feels_like.toFixed(1) + ' °C';
    document.getElementById('humidity').textContent = data.humidity + '%';
    document.getElementById('pressure').textContent = data.pressure + ' hPa';
    document.getElementById('wind-speed').textContent = data.wind_speed.toFixed(1) + ' m/s';
    document.getElementById('visibility').textContent = (data.visibility / 1000).toFixed(1) + ' km';

    weatherCard.classList.remove('hidden');
}

function setLoading(on) {
    searchBtn.disabled = on;
    loadingEl.classList.toggle('hidden', !on);
}

function showError(msg) {
    errorMsg.textContent = msg;
    errorMsg.classList.remove('hidden');
}

function clearError() {
    errorMsg.textContent = '';
    errorMsg.classList.add('hidden');
}
