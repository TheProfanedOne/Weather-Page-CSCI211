"use strict";

/*
    Author: Matthew Mousseau
    Date of Initial Creation: 11/09/2022
    Filename: script.js
*/

var datasets;
let threeDayForecast = true;
const threeDayWidth  = 702;
const fiveDayWidth   = 1170;

window.addEventListener('load', async () => {
    const darkTheme  = await init();
    const main       = document.querySelector('main');
    const apply      = document.querySelector("#apply");
    const refresh    = document.querySelector("#refresh");
    const darkToggle = document.querySelector('#darkToggle');
    const threeDay   = document.querySelector("#threeDay");
    const fiveDay    = document.querySelector("#fiveDay");

    apply.addEventListener('click', evt => {
        const data = new FormData(document.forms.cityForm);
        const city = cityValues.indexOf(data.get('selected_city'));
        showForecast(city);

        evt.target.style.backgroundColor = "limegreen";
        setTimeout(() => evt.target.removeAttribute('style'), 200);
    }); 

    refresh.addEventListener('click', async evt => {
        const newDate = new Date();
        await genForecastData(newDate);
        apply.dispatchEvent(fakeEvent(evt.target));
    });

    darkToggle.addEventListener('click', evt => {
        darkTheme.disabled = !darkTheme.disabled;
        evt.target.style.backgroundColor = 'limegreen';
        setTimeout(() => evt.target.removeAttribute('style'), 200);
    });

    threeDay.addEventListener('click', evt => {
        threeDayForecast  = true;
        threeDay.disabled = true;
        fiveDay.disabled  = false;
        main.style.width  = `${threeDayWidth}px`;
        apply.dispatchEvent(fakeEvent(evt.target));
    });

    fiveDay.addEventListener('click', evt => {
        threeDayForecast  = false;
        fiveDay.disabled  = true;
        threeDay.disabled = false;
        main.style.width  = `${fiveDayWidth}px`;
        apply.dispatchEvent(fakeEvent(evt.target));
    });
});

async function init() {
    const thisDate = new Date();

    const darkTheme = document.createElement('link');
    darkTheme.setAttribute('id', 'darkTheme');
    darkTheme.setAttribute('rel', 'stylesheet');
    darkTheme.setAttribute('href', './css/dark.css');
    darkTheme.setAttribute('type', 'text/css');
    darkTheme.setAttribute('disabled', 'disabled');
    darkTheme.disabled = true;

    await genForecastData(thisDate);

    document.head.appendChild(darkTheme);
    const mainElement = document.querySelector('main');
    mainElement.style.width = `${threeDayWidth}px`;
    document.getElementById('body-title').innerHTML = 'CSCI-211 &nbsp;&mdash;&nbsp; Weather Page';
    showTopLine(thisDate);
    showForecast(0);
    showBottomLine();
    document.body.removeAttribute('style');

    return darkTheme;
}

function fakeEvent(value) {
    return Object.defineProperty(new MouseEvent('click'), 'target', { value });
}
