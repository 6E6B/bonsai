/** What a purchase leaves: the balance after it, and how many Robux are missing (0 when there
 * are enough). `after` is null while the balance is unknown. */
export function afterPurchase(balance: number | null, price: number): { after: number | null; short: number } {
  if (balance === null) return { after: null, short: 0 }
  return { after: Math.max(0, balance - price), short: Math.max(0, price - balance) }
}
