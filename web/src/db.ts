import Dexie from 'dexie';
import type { Table } from 'dexie';
import type { DatasetInfo } from '@minerva/dataset-client';

export class MinervaDexie extends Dexie {
  datasets!: Table<DatasetInfo>;

  constructor() {
    super('minerva');
    this.version(1).stores({
      datasets: '++name, tableName', // Primary key and indexed props
    });
  }
}

export const db = new MinervaDexie();
