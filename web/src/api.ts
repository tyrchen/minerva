import { DatasetServiceClient, ListDatasetCommand, SigninCommand, QueryDatasetCommand } from '@minerva/dataset-client';
import type { DatasetInfo } from '@minerva/dataset-client';
import Cookie from 'js-cookie';
import { db } from './db';
import { tableFromIPC, Table } from 'apache-arrow';

const ENDPOINT = 'https://ds-api.staging.sandbox.tubi.io/api';
// const ENDPOINT = 'http://localhost:3000/api';

export const token: string = Cookie.get('token');

const client = new DatasetServiceClient({
  endpoint: ENDPOINT,
  token: { token },
});

export const signin = async (username: string, password: string) => {
  const command = new SigninCommand({
    username,
    password,
  });
  try {
    const ret = await client.send(command);
    Cookie.set('token', ret.token);
  } catch (e) {
    console.log('Failed to signin:', e);
  }
};

export const loadDatasets = async () => {
  try {
    const command = new ListDatasetCommand({});
    const items = (await client.send(command)).items || [];
    console.log('loaded datasets:', items.length);
    db.datasets.bulkPut(items);
    return items;
  } catch (e) {
    console.log('Failed to load datasets:', e);
    return [];
  }
};

export const queryDataset = async (sql: string, dataset: DatasetInfo) => {
  const command = new QueryDatasetCommand({
    id: dataset.name,
    sql,
  });
  const data = (await client.send(command)).data || new Uint8Array();

  console.log('data:', data.length);
  return tableFromIPC(data);
};

export const tableToJson = (table: Table) => {
  const ret = [] as object[];
  for (const item of table) {
    const row = {};
    for (const key in item) {
      if (item[key] instanceof Uint8Array) {
        row[key] = btoa(item[key]);
      } else {
        row[key] = item[key];
      }
    }
    ret.push(row);
  }
  return ret;
};

export const tableToColumns = (table: Table, names: string[]) => {
  const ret = {};
  for (const name of names) {
    const vector = table.getChild(name);
    const type = typeof vector?.get(0);
    if (type === 'bigint' || type === 'number') {
      ret[name] = [];
    } else {
      console.log(`skip column ${name} with type ${type}`);
      continue;
    }

    for (const item of vector) {
      if (typeof item === 'bigint') {
        ret[name].push(Number(item));
      } else {
        ret[name].push(item);
      }
    }
  }
  console.log('ret:', ret);

  return ret;
};

export const setCurrentDataset = async (name: string) => {
  Cookie.set('dataset', name);
};

export const getCurrentDataset = async (): Promise<DatasetInfo | undefined> => {
  const name = Cookie.get('dataset');
  if (!name) {
    return undefined;
  }
  return await db.datasets.get(name);
};
