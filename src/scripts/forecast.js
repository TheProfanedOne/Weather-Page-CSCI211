const apiLocations = [
    "latitude=47.50&longitude=-111.30",
    "latitude=51.51&longitude=-0.13",
    "latitude=48.85&longitude=2.35",
    "latitude=-22.91&longitude=-43.18",
    "latitude=-33.87&longitude=151.21",
    "latitude=40.71&longitude=-74.01",
];

function getApiDate(date) {
    const offset = date.getTimezoneOffset();
    const tempDate = new Date(date.getTime() - (offset * 60 * 1000));
    return tempDate.toISOString().split('T')[0];
}

function getApiURL(start, end, loc) {
    const apiDom = 'https://api.open-meteo.com/v1/forecast?';
    const apiModel = '&models=best_match';
    const apiDaily = '&daily=weathercode,temperature_2m_max,temperature_2m_min';
    const apiUnits = '&temperature_unit=fahrenheit';
    const apiTime = '&timeformat=unixtime&timezone=auto';
    const apiDates = `&start_date=${getApiDate(start)}&end_date=${getApiDate(end)}`;

    return `${apiDom}${loc}${apiModel}${apiDaily}${apiUnits}${apiTime}${apiDates}`;
}

async function getWeatherData(start, end, loc) {
    return fetch(getApiURL(start, end, loc), {}).then(res => res.json());
}

async function genForecastData(thisDate, numDays = 3) {
    const sDate = new Date(thisDate.getTime() + (1000 * 60 * 60 * 24 * 1));
    const eDate = new Date(thisDate.getTime() + (1000 * 60 * 60 * 24 * numDays));

    datasets = [];
    for (const loc of apiLocations) {
        const data = await getWeatherData(sDate, eDate, loc);
        datasets.push(data);
    }
}

function genForecasts(city) {
    const forecasts = [];
    const data = datasets[city];
    const len = data.daily.time.length;

    for (let i = 0; i < len; i++) forecasts.push({
        date: getDateStr(new Date(data.daily.time[i] * 1000)),
        summ: `${data.daily.weathercode[i]}`,
        icon: 'images/icons/sun.png',
        temp: `${data.daily.temperature_2m_min[i]} / ${data.daily.temperature_2m_max[i]}`,
        desc: "Sunny and Warm",
    });

    return forecasts
}

function showForecast(city) {
    const forecasts = genForecasts(city);
    const days = document.getElementsByClassName('forecast');

    for (let i = 0; i < days.length; i++) {
        const day = days[i];
        day.innerHTML = "";

        const forecast = forecasts[i];

        const date = document.createElement('p');
        date.classList.add('forecast-contents');
        date.textContent = forecast.date;

        const summ = document.createElement('p');
        summ.classList.add('forecast-contents');
        summ.textContent = forecast.summ;

        const icon = document.createElement('div');
        icon.classList.add('forecast-contents');
        icon.style.display = "flex";
        icon.style.justifyContent = "center";
        icon.innerHTML = `<img src="${forecast.icon}" width="32">`;

        const temp = document.createElement('p');
        temp.classList.add('forecast-contents');
        temp.textContent = forecast.temp;

        const desc = document.createElement('p');
        desc.classList.add('forecast-contents');
        desc.textContent = forecast.desc;
        
        day.appendChild(date);
        day.appendChild(summ);
        day.appendChild(icon);
        day.appendChild(temp);
        day.appendChild(desc);
    }
}