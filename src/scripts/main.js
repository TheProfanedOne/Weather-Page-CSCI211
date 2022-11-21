"use strict";

/*
    Author: Matthew Mousseau
    Date of Initial Creation: 11/09/2022
    Filename: script.js
*/

var datasets;

window.addEventListener('load', async () => {
    const thisDate = new Date();

    await genForecastData(thisDate);

    document.getElementById('body-title').innerHTML = 'CSCI-211 &mdash; Weather Page';
    showTopLine(thisDate);
    document.getElementById('big-image').src =
        "images/big-image/2102.i518.007_sky_cloud_summer_landscape.jpg";
    showForecast(0);

    document.body.removeAttribute('style');
});
