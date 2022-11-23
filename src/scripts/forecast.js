"use strict";

const apiLocations = [
    "latitude=47.50&longitude=-111.30",
    "latitude=51.51&longitude=-0.13",
    "latitude=48.85&longitude=2.35",
    "latitude=-22.91&longitude=-43.18",
    "latitude=-33.87&longitude=151.21",
    "latitude=40.71&longitude=-74.01",
];

const bigImageURLs = [
    './images/big-image/Great_Falls.jpg',
    './images/big-image/London.jpg',
    './images/big-image/Paris.jpg',
    './images/big-image/Rio.jpg',
    './images/big-image/Sydney.jpg',
    './images/big-image/New_York.jpg',
];

function getISODateStr(date) {
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
    const apiDates = `&start_date=${getISODateStr(start)}&end_date=${getISODateStr(end)}`;

    return `${apiDom}${loc}${apiModel}${apiDaily}${apiUnits}${apiTime}${apiDates}`;
}

async function getWeatherData(start, end, loc) {
    return fetch(getApiURL(start, end, loc), {}).then(res => res.json());
}

async function genForecastData(thisDate) {
    const sDate = new Date(thisDate.getTime() + (1000 * 60 * 60 * 24 * 1));
    const eDate = new Date(thisDate.getTime() + (1000 * 60 * 60 * 24 * 5));

    datasets = [];
    for (const loc of apiLocations) {
        const data = await getWeatherData(sDate, eDate, loc);
        datasets.push(data);
    }
}

function parseCodeSumm(code) {
    return (
        code === 0  ? 'Clear Sky'                     :
        code === 1  ? 'Mainly Clear'                  :
        code === 2  ? 'Partly Cloudy'                 :
        code === 3  ? 'Overcast'                      :
        code === 45 ? 'Foggy'                         :
        code === 48 ? 'Depositing Rime Fog'           :
        code === 51 ? 'Slight Drizzle'                :
        code === 53 ? 'Moderate Drizzle'              :
        code === 55 ? 'Heavy Drizzle'                 :
        code === 56 ? 'Slight Freezing Drizzle'       :
        code === 57 ? 'Heavy Freezing Drizzle'        :
        code === 61 ? 'Slight Rain'                   :
        code === 63 ? 'Moderate Rain'                 :
        code === 65 ? 'Heavy Rain'                    :
        code === 66 ? 'Slight Freezing Rain'          :
        code === 67 ? 'Heavy Freezing Rain'           :
        code === 71 ? 'Slight Snow Fall'              :
        code === 73 ? 'Moderate Snow Fall'            :
        code === 75 ? 'Heavy Snow Fall'               :
        code === 77 ? 'Snow Grains'                   :
        code === 80 ? 'Slight Rain Showers'           :
        code === 81 ? 'Moderate Rain Showers'         :
        code === 82 ? 'Violent Rain Showers'          :
        code === 85 ? 'Slight Snow Showers'           :
        code === 86 ? 'Heavy Snow Showers'            :
        code === 95 ? 'Thunderstorm'                  :
        code === 96 ? 'Thunderstorm with Slight Hail' :
        code === 99 ? 'Thunderstorm with Heavy Hail'
                    : 'Unrecognized Weathercode'
    );
}

function parseCodeIcon(code) {
    return (
        code === 0  ? 'wi-day-sunny'          :
        code === 1  ? 'wi-day-sunny-overcast' :
        code === 2  ? 'wi-day-cloudy'         :
        code === 3  ? 'wi-cloudy'             :
        code === 45 || code === 48
                    ? 'wi-day-fog'            :
        code === 51 || code === 53 || code === 55
                    ? 'wi-day-sprinkle'       :
        code === 56 || code === 57
                    ? 'wi-day-sleet'          :
        code === 61 || code === 63 || code === 65
                    ? 'wi-day-rain'           :
        code === 66 || code === 67
                    ? 'wi-day-rain-mix'       :
        code === 71 || code === 73 || code === 75 || code === 77
                    ? 'wi-day-snow'           :
        code === 80 || code === 81 || code === 82
                    ? 'wi-day-showers'        :
        code === 85 || code === 86
                    ? 'wi-day-snow-wind'      :
        code === 95 ? 'wi-day-thunderstorm'   :
        code === 96 || code === 99
                    ? 'wi-day-sleet-storm'
                    : 'wi-alien'
    );
}

function parseTemp(max, min) {
    const avg = (max + min) / 2.0
    return (
        avg >= 140 ? 'Impossibly Hot'   :
        avg >= 120 ? 'Dangerously Hot'  :
        avg >= 110 ? 'Extremely Hot'    :
        avg >= 100 ? 'Very Hot'         :
        avg >= 90  ? 'Hot'              :
        avg >= 80  ? 'Very Warm'        :
        avg >= 70  ? 'Warm'             :
        avg >= 60  ? 'Moderate'         :
        avg >= 50  ? 'Cool'             :
        avg >= 40  ? 'Very Cool'        :
        avg >= 30  ? 'Slightly Cold'    :
        avg >= 20  ? 'Moderately Cold'  :
        avg >= 10  ? 'Cold'             :
        avg >= 0   ? 'Very Cold'        :
        avg >= -10 ? 'Extremely Cold'   :
        avg >= -30 ? 'Dangerously Cold'
                   : 'Impossibly Cold'
    );
}

function genForecasts(city, len) {
    const data  = datasets[city];
    const time  = data.daily.time;
    const code  = data.daily.weathercode;
    const tMin  = data.daily.temperature_2m_min;
    const tMax  = data.daily.temperature_2m_max;
    const tUnit = data.daily_units.temperature_2m_max;

    const forecasts = [];
    for (let i = 0; i < len; i++) {
        const summ = parseCodeSumm(code[i]);
        if (summ === 'Unrecognized Weathercode') console.debug(code[i]);
        forecasts.push({
            date: `${getISODateStr(new Date(time[i] * 1000))} (${data.timezone_abbreviation})`,
            summ,
            icon: `wi ${parseCodeIcon(code[i])}`,
            temp: `${tMax[i]}${tUnit} / ${tMin[i]}${tUnit}`,
            desc: parseTemp(tMax[i], tMin[i]),
        });
    }
    

    return forecasts;
}

function showForecast(city) {
    const len = threeDayForecast ? 3 : 5;
    const forecasts = genForecasts(city, len);

    const bigImage = document.querySelector('#big-image');
    if (len === 5 && city !== 5) {
        const vertical=city===0?60:city===1?45:city===2?25:city===3?30:65;
        bigImage.style.objectPosition = `center ${vertical}%`;
    } else {
        bigImage.style.objectPosition = 'center';
    }
    bigImage.src = bigImageURLs[city];

    const container = document.getElementById('forecast-container');
    container.style.gridTemplateColumns = `repeat(${len}, 1fr)`;
    container.innerHTML = "";

    const dayWidth = threeDayForecast ? '98%' : '99%';

    for (let i = 0; i < len; i++) {
        const forecast = forecasts[i];
        const day = document.createElement('section');
        day.style.width = dayWidth;
        day.classList.add('forecast');

        const date = document.createElement('p');
        date.classList.add('forecast-contents');
        date.textContent = forecast.date;
        day.appendChild(date);

        const summ = document.createElement('p');
        summ.classList.add('forecast-contents');
        summ.textContent = forecast.summ;
        day.appendChild(summ);

        const icon = document.createElement('p');
        icon.classList.add('forecast-contents');
        icon.innerHTML = `<i class="${forecast.icon}"></i>`;
        icon.style.fontSize = "1.1rem";
        day.appendChild(icon);

        const temp = document.createElement('p');
        temp.classList.add('forecast-contents');
        temp.textContent = forecast.temp;
        day.appendChild(temp);

        const desc = document.createElement('p');
        desc.classList.add('forecast-contents');
        desc.textContent = forecast.desc;
        day.appendChild(desc);
        
        container.appendChild(day);
    };
}