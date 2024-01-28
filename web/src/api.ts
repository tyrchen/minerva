import { DatasetServiceClient, ListDatasetCommand, SigninCommand, QueryDatasetCommand } from '@minerva/dataset-client';
import type { DatasetInfo } from '@minerva/dataset-client';
import Cookie from 'js-cookie';
import { db } from './db';
import { tableFromIPC } from 'apache-arrow';

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
  const table = tableFromIPC(data);
  return JSON.parse(table.toString());
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
