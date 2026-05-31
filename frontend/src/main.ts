import { bootstrapApplication } from "@angular/platform-browser";
import { provideRouter, type Routes } from "@angular/router";
import { provideHttpClient } from "@angular/common/http";
import { AppComponent } from "./app/app.component";

const routes: Routes = [{ path: "", component: AppComponent }];

bootstrapApplication(AppComponent, {
  providers: [provideRouter(routes), provideHttpClient()],
}).catch((error) => console.error(error));
