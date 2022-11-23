"use strict";

function showBottomLine() {
    showThemeButton();
    showTimeButtons();
}

function showThemeButton() {
    const sect = document.querySelector('#themeSect');

    const darkToggle = document.createElement('button');
    darkToggle.setAttribute('id', 'darkToggle');
    darkToggle.setAttribute('type', 'button');
    darkToggle.innerText = "Toggle Dark Theme";

    sect.appendChild(darkToggle);
}

function showTimeButtons() {
    const sect = document.querySelector('#timeSect');

    const b3 = document.createElement('button');
    b3.setAttribute('id', 'threeDay');
    b3.setAttribute('type', 'button');
    b3.setAttribute('disabled', 'disabled');
    b3.disabled = true;
    b3.innerText = "Three-Day Forecast";

    const b5 = document.createElement('button');
    b5.setAttribute('id', 'fiveDay');
    b5.setAttribute('type', 'button');
    b5.setAttribute('disabled', 'disabled');
    b5.disabled = false;
    b5.innerText = "Five-Day Forecast";

    sect.appendChild(b3);
    sect.appendChild(b5);
}
