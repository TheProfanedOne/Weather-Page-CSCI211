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

function main() {
    const thisDate  = new Date();
    const thisDay   = dayNames[thisDate.getDay()];
    const thisMonth = monthNames[thisDate.getMonth()];

    const dateStr = `${thisDay} ${thisMonth} ${thisDate.getDate()}, ${thisDate.getFullYear()}`;

    const dateArea = document.getElementById('dateArea');
    const dateText = document.createElement('p');
    dateText.textContent = dateStr;
    dateArea.appendChild(dateText);
}

main();

