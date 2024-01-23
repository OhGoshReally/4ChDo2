import { createApp } from "vue";
import "./styling/styles.css";
import "./styling/styles.scss";
import App from "./App.vue";

/* import the fontawesome core */
import { library } from "@fortawesome/fontawesome-svg-core";

/* import font awesome icon component */
import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";

/* import specific icons */
import { faCaretLeft, faCaretRight, faImage } from "@fortawesome/free-solid-svg-icons";

/* add icons to the library */
library.add(faCaretLeft, faCaretRight, faImage);

createApp(App)
    .component('font-awesome-icon', FontAwesomeIcon)
    .mount("#app");
