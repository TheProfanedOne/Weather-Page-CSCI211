const dayNames = [
    "Sunday", "Monday", "Tuesday", "Wednesday",
    "Thursday", "Friday", "Saturday"
];

const monthNames = [
    "Jan", "Feb", "Mar", "Apr",
    "May", "Jun", "Jul", "Aug",
    "Sep", "Oct", "Nov", "Dec"
];

const cityNames = [
    "Great Falls, MT, US",
    "London, UK",
    "Paris, FR",
    "Rio de Janeiro, BR",
    "Sydney, AU",
    "New York, NY, US",
];

const cityValues = [
    "great_falls",
    "london",
    "paris",
    "rio_de_janeiro",
    "sydney",
    "new_york",
];

function showTopLine(thisDate) {
    showDate(thisDate);
    showCityForm();
}

function getDateStr(thisDate) {
    const thisDay = dayNames[thisDate.getDay()];
    const thisMonth = monthNames[thisDate.getMonth()];
    return `${thisDay} ${thisMonth} ${thisDate.getDate()}, ${thisDate.getFullYear()}`;
}

function showDate(thisDate) {
    const date = document.getElementById('date');
    date.textContent = `Today is: ${getDateStr(thisDate)}`;
}

function showCityForm() {
    const form = document.getElementById('cityForm');
    const fieldset = document.createElement('fieldset');
    const label = document.createElement('label');
    const select = document.createElement('select');

    label.textContent = "City: ";
    label.setAttribute('for', 'cityMenu');
    select.setAttribute('id', 'cityMenu');
    select.setAttribute('name', 'selected_city');
    select.setAttribute('size', '1');

    for (let i = 0; i < cityNames.length; i++) {
        const opt = document.createElement('option');
        opt.value = cityValues[i];
        opt.text = cityNames[i];
        select.appendChild(opt);
    }

    fieldset.appendChild(label);
    fieldset.appendChild(select);
    form.appendChild(fieldset);

    const apply = document.createElement('button');
    apply.setAttribute('id', 'apply');
    apply.setAttribute('type', 'button');
    apply.innerText = 'Apply';
    apply.addEventListener('click', evt => {
        const c_form = document.forms.cityForm;
        const data = new FormData(c_form);
        const city = cityValues.indexOf(data.get('selected_city'));
        showForecast(city);

        evt.target.style.backgroundColor = "limegreen";
        setTimeout(() => evt.target.removeAttribute('style'), 200);
    });

    const refresh = document.createElement('button');
    refresh.setAttribute('id', 'refresh');
    refresh.setAttribute('type', 'button');
    refresh.innerText = 'Refresh';
    refresh.addEventListener('click', async evt => {
        const newDate = new Date();

        await genForecastData(newDate); let value = evt.target;
        apply.dispatchEvent(Object.defineProperty(evt, 'target', { value }));
    });

    const sect = document.querySelector('#topLine > section');
    sect.appendChild(apply);
    sect.appendChild(refresh);
}
