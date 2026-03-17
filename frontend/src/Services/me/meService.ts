import { http } from "../http";
import type { BootstrapResponse } from "./meTypes";

export async function bootstrap(): Promise<BootstrapResponse> {
  return http.get<BootstrapResponse>("/v1/me/bootstrap");
}
