import { http } from "../http";

export type Invoice = {
  id: number;
  user_id: number;
  stripe_invoice_id: string;
  amount: number;
  currency: string;
  status: string;
  hosted_url: string | null;
  created_at: string;
};

export type InvoiceListResponse = {
  invoices: Invoice[];
  total: number;
};

export async function listMyInvoices(offset = 0, limit = 20): Promise<InvoiceListResponse> {
  return http.get<InvoiceListResponse>(`/v1/invoices/mine?offset=${offset}&limit=${limit}`);
}
