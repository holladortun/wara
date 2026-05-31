import { Component, inject } from "@angular/core";
import { CommonModule } from "@angular/common";
import { FormsModule } from "@angular/forms";
import { HttpClient } from "@angular/common/http";

type Stat = {
  label: string;
  value: string;
  detail: string;
};

@Component({
  selector: "wara-root",
  standalone: true,
  imports: [CommonModule, FormsModule],
  templateUrl: "./app.component.html",
})
export class AppComponent {
  private readonly http = inject(HttpClient);

  readonly isInviteAccept = window.location.pathname === "/accept-invite";
  readonly inviteToken =
    new URLSearchParams(window.location.search).get("token") ?? "";
  invitePassword = "";
  inviteStatus: "idle" | "submitting" | "accepted" | "error" = "idle";
  inviteMessage = "";

  readonly stats: Stat[] = [
    { label: "Servers", value: "0", detail: "Add Docker hosts over SSH" },
    { label: "Projects", value: "0", detail: "Group apps by workspace" },
    { label: "Deployments", value: "0", detail: "Temporal-backed jobs" },
    { label: "Telemetry", value: "Off", detail: "Platform-only and opt-in" },
  ];

  readonly nav = [
    "Servers",
    "Projects",
    "Templates",
    "Services",
    "Deployments",
    "Domains",
    "Credentials",
    "Telemetry",
  ];

  acceptInvite(): void {
    if (this.invitePassword.length < 8) {
      this.inviteStatus = "error";
      this.inviteMessage = "Use at least 8 characters.";
      return;
    }

    this.inviteStatus = "submitting";
    this.inviteMessage = "";
    this.http
      .post<{ token: string }>("/api/v1/auth/invites/accept", {
        token: this.inviteToken,
        password: this.invitePassword,
      })
      .subscribe({
        next: (response) => {
          localStorage.setItem("wara_token", response.token);
          this.inviteStatus = "accepted";
          this.inviteMessage = "Password created. Redirecting to Wara.";
          window.setTimeout(() => window.location.assign("/"), 800);
        },
        error: () => {
          this.inviteStatus = "error";
          this.inviteMessage = "This invite is invalid or expired.";
        },
      });
  }
}
