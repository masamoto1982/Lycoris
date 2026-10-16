// コアの Snapshot（core/src/store.rs）を TypeScript から読むための型。

export type Side = "staff" | "customer";
export type Mode = "attended" | "semi_self" | "full_self";
export type Phase = "registering" | "paying";
export type Method = "cash" | "emoney" | "qr";
export type StaffState = "absent" | "locked" | "ready";
export type FfState = "waiting" | "presented" | "received";

export interface ActionState {
  action: string;
  allowed: boolean;
  reason: string | null;
}

export type LineKind =
  | { kind: "item" }
  | { kind: "ff"; state: FfState }
  | { kind: "service"; done: boolean };

export type StaffLine = LineKind & {
  id: number;
  code: string;
  name: string;
  unit_price: number;
  qty: number;
  category: string;
  amount: number;
  age: number | null;
};

export type CustomerLine = LineKind & { id: number; name: string; qty: number; amount: number };

export interface Call {
  id: number;
  register: number;
  kind: string;
  label: string;
  created_at: number;
  detail: string;
}

export interface Receipt {
  no: number;
  total: number;
  method: string;
  paid: number;
  change: [number, number][];
  change_total: number;
  items: number;
  point_card: string | null;
  points: number;
}

export interface StaffView {
  state: StaffState;
  name: string | null;
  role: "crew" | "owner" | null;
  phase: Phase;
  lines: StaffLine[];
  total: number;
  tax8: number;
  tax10: number;
  point_card: string | null;
  required_age: number | null;
  age_verified: boolean;
  age_pending: boolean;
  method: Method | null;
  inserted: number;
  due: number;
  payment_blocker: string | null;
  calls: Call[];
  actions: ActionState[];
  drawer_total: number;
  mat_down: string[];
}

export interface CustomerView {
  phase: Phase;
  lines: CustomerLine[];
  total: number;
  inserted: number;
  due: number;
  point_card: boolean;
  age_check: number | null;
  staff_called: boolean;
  staff_present: boolean;
  method: Method | null;
  payment_blocker: string | null;
  actions: ActionState[];
}

export interface RegisterView {
  id: number;
  /** 今、画面を操作できる側（もう一方は表示だけ） */
  active_side: Side;
  mode: Mode;
  mode_label: string;
  staff: StaffView;
  customer: CustomerView;
  last_receipt: Receipt | null;
}

export interface AgeRule {
  id: number;
  target: { kind: "category"; category: string } | { kind: "product"; code: string };
  label: string;
  min_age: number;
  legal: boolean;
}

export interface Snapshot {
  now: number;
  registers: RegisterView[];
  mats: { id: string; buzzing: boolean; link_ok: boolean; rings: number; offline: boolean }[];
  mat_links: { id: string; link_ok: boolean; acked_seq: number; sent: number; retries: number }[];
  bell: { seq: number; ring_token: number; open_calls: number };
  calls: Call[];
  call_history: { id: number; register: number; label: string; created_at: number; resolved_at: number }[];
  age_rules: { rules: AgeRule[]; history: { at: number; by: string; change: string }[] };
  audit: { at: number; register: number; staff: string; operation: string; amount: number; result: string }[];
  ff_stock: { code: string; name: string; price: number; stock: number }[];
  mat_loss: number;
  accounts: { id: string; barcode: string; name: string; role: string }[];
  catalog: { code: string; name: string; price: number; category: string; category_label: string; age: number | null; legal_age: number | null; store_age: number | null }[];
  categories: [string, string][];
}

export type Command = { type: string; [k: string]: unknown };
