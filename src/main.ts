import { createApp } from 'vue';
import { createPinia } from 'pinia';

import '@/assets/icons/sass/weather-icons.min.scss';
import '@/assets/icons/sass/weather-icons-wind.min.scss';

import App from './App.vue';
import router from './router';

const app = createApp(App);

app.use(createPinia());
app.use(router);

app.mount('#app');
