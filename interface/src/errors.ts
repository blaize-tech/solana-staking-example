export abstract class stakingError extends Error {}

export class AccountNotFoundError extends stakingError {
  constructor(public accountName = '') {
    super(`Account ${accountName} not found`);
  }
}
