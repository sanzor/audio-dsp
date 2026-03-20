import { http } from "../http";

export type PurchasedProduct = {
  id: number;
  user_id: number;
  product_id: number;
  purchased_at: string;
};

export async function listMyPurchases(): Promise<PurchasedProduct[]> {
  return http.get<PurchasedProduct[]>("/v1/purchased-products/mine");
}
