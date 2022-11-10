"use strict";

/*
    Author: Matthew Mousseau
    Date of Initial Creation: 11/09/2022
    Filename: script.js

*/

const dayNames = [
    "Sunday", "Monday", "Tuesday", "Wednesday",
    "Thursday", "Friday", "Saturday"
];

const monthNames = [
    "Jan", "Feb", "Mar", "Apr",
    "May", "Jun", "Jul", "Aug",
    "Sep", "Oct", "Nov", "Dec"
];

function showDate(thisDate) {
    const thisDay   = dayNames[thisDate.getDay()];
    const thisMonth = monthNames[thisDate.getMonth()];

    const dateStr = `${thisDay} ${thisMonth} ${thisDate.getDate()}, ${thisDate.getFullYear()}`;

    const dateArea = document.getElementById('dateArea');
    const dateText = document.createElement('p');
    dateText.textContent = dateStr;
    dateArea.appendChild(dateText);
}

function generateForecasts(thisDate) {
    let forecasts = [];

    if (thisDate instanceof Date) {
        Function.prototype(); // temporary no-op
    }

    forecasts.push({
        summ: "Sunny",
        icon: "images/icons/sun.png",
        temp: "70 / 60",
        desc: "Warm and Sunny"
    });

    forecasts.push({
        summ: "Partly Cloudy",
        icon: "images/icons/cloudy-weather.png",
        temp: "63 / 44",
        desc: "Cooler with Partial Cloud Cover"
    });

    forecasts.push({
        summ: "Raining",
        icon: "images/icons/rainy-day.png",
        temp: "60 / 53",
        desc: "Cool with Chance of Rain"
    });

    return forecasts;
}

function showForecast(thisDate) {
    const forecasts = generateForecasts(thisDate);
    const days = document.getElementsByClassName('forecast');

    const big_image = document.getElementById('big-image');
    big_image.src = "images/big-image/2102.i518.007_sky_cloud_summer_landscape.jpg";

    for (let i = 0; i < days.length; i++) {
        const day = days[i];
        const forecast = forecasts[i];

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

        day.appendChild(summ);
        day.appendChild(icon);
        day.appendChild(temp);
        day.appendChild(desc);
    }
}

window.onload = () => {
    const thisDate  = new Date();

    showDate(thisDate);
    showForecast(thisDate);
};

