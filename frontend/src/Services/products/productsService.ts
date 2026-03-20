import { http } from "../http";

export type Product = {
  id: number;
  name: string;
  description: string | null;
  tier: string;
  price_cents: number;
  currency: string;
  is_active: boolean;
  created_at: string;
};

export async function listProducts(activeOnly = true): Promise<Product[]> {
  return http.get<Product[]>(`/v1/products?active_only=${activeOnly}`);
}

export async function getProduct(id: number): Promise<Product> {
  return http.get<Product>(`/v1/products/${id}`);
}
