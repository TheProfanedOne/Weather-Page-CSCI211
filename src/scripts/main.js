"use strict";

/*
    Author: Matthew Mousseau
    Date of Initial Creation: 11/09/2022
    Filename: script.js
*/

let datasets = [];
let threeDayForecast = true;
const threeDayWidth  = 702;
const fiveDayWidth   = 1170;

window.addEventListener('load', () => {
    const thisDate  = new Date();
    const main      = document.querySelector('main');
    const darkTheme = document.createElement('link');
    darkTheme.setAttribute('id', 'darkTheme');
    darkTheme.setAttribute('rel', 'stylesheet');
    darkTheme.setAttribute('href', './css/dark.css');
    darkTheme.setAttribute('type', 'text/css');
    darkTheme.setAttribute('disabled', 'disabled');
    darkTheme.disabled = true;
    document.head.appendChild(darkTheme);

    main.style.width = `${threeDayWidth}px`;

    showTopLine(thisDate);
    showForecast(0, true);
    showBottomLine();

    genForecastData(thisDate).then(_ => {
        showForecast(0, false);

        const sel_city   = document.getElementsByTagName("select")[0];
        const buttons    = document.getElementsByTagName("button");
        const refresh    = buttons.namedItem("refresh");
        const darkToggle = buttons.namedItem("darkToggle");
        const threeDay   = buttons.namedItem("threeDay");
        const fiveDay    = buttons.namedItem("fiveDay");

        sel_city.addEventListener('change', evt => {
            showForecast(sel_city.selectedIndex, false);
            if (evt.detail) flashButton(evt.detail);
        }, { passive: true });
    
        refresh.addEventListener('click', evt => {
            const newDate = new Date();
            showDate(thisDate);
            showForecast(sel_city.selectedIndex, true);
            genForecastData(newDate).then(
                _ => sel_city.dispatchEvent(fakeEvent(evt.target))
            );
        }, { passive: true });
    
        darkToggle.addEventListener('click', evt => {
            darkTheme.disabled = !darkTheme.disabled;
            flashButton(evt.target);
        }, { passive: true});
    
        threeDay.addEventListener('click', evt => {
            threeDayForecast  = true;
            threeDay.disabled = true;
            fiveDay.disabled  = false;
            main.style.width  = `${threeDayWidth}px`;
            sel_city.dispatchEvent(fakeEvent(evt.target));
        }, { passive: true });
    
        fiveDay.addEventListener('click', evt => {
            threeDayForecast  = false;
            fiveDay.disabled  = true;
            threeDay.disabled = false;
            main.style.width  = `${fiveDayWidth}px`;
            sel_city.dispatchEvent(fakeEvent(evt.target));
        }, { passive: true });
    });
}, { passive: true });

function flashButton(target) {
    target.style.backgroundColor = "limegreen";
    setTimeout(() => target.removeAttribute("style"), 200);
}

function fakeEvent(detail) {
    return new CustomEvent("change", { detail });
}
