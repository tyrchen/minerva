import type { App } from 'vue';
import PrimeVue from 'primevue/config';
import Lara from '~/presets/wind'; //import preset

export default (app: App) => {
  app.use(PrimeVue, {
    unstyled: true,
    pt: Lara,
  });
};
