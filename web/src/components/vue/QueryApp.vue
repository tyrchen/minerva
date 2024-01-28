<template>
  <div class="flex flex-col flex-1">
    <div class="flex flex-1 overflow-y-auto">
      <!-- left column area -->
      <LeftNav :nodes="nodes" v-model:dataset="selectedDataset" />
      <main class="flex flex-1 overflow-y-auto">
        <!-- Main area -->

        <div class="flex flex-col flex-1 p-2 overflow-y-auto">
          <div>
            <div class="my-4 flex items-center cursor-pointer" @click="showFields = !showFields">
              <IconCaretRightFilled v-if="!showFields" />
              <IconCaretDownFilled v-if="showFields" />
              <h1 class="text-2xml">{{ selectedDataset.name }}</h1>
              <span class="inline-block mx-4 text-sm text-gray-500"
                >(last modified: {{ selectedDataset.lastModified }})</span
              >
            </div>

            <DataTable
              class="my-4 max-w-7xl overflow-x-scroll"
              :value="selectedDataset.fields"
              size="small"
              scrollable
              scrollHeight="400px"
              stripedRows
              v-show="showFields"
            >
              <Column field="name" header="Name"></Column>
              <Column field="type" header="Type"></Column>
              <Column field="nullable" header="Nullable"></Column>
            </DataTable>
          </div>
          <Textarea v-model="query" class="w-full h-48"> </Textarea>
          <div class="flex items-center">
            <Button class="w-48 my-4" label="Execute!" raised size="large" @click="executeQuery">
              <IconSearch v-if="!isQuerying" />
              <IconRefresh v-if="isQuerying" class="animate-spin" />
              <span class="px-3">Execute!</span>
            </Button>
            <span class="inline-block mx-4 text-sm text-gray-500" v-text="queryStatus"></span>
          </div>

          <DataTable
            v-show="queryRsult.length > 0"
            class="my-4 max-w-7xl overflow-x-scroll border"
            paginator
            :rows="50"
            :rowsPerPageOptions="[50, 100, 200]"
            :value="queryRsult"
            size="small"
            scrollable
            scrollHeight="400px"
            stripedRows
          >
            <Column class="min-w-64" v-for="col in queryColumns" :field="col.name" :header="col.label"></Column>
          </DataTable>
        </div>
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
import { nextTick, onMounted, ref } from 'vue';

import { db } from '../../db';
import { getCurrentDataset, loadDatasets } from '../../api';
import type { DatasetInfo } from '@minerva/dataset-client';
import type { TreeNode } from 'primevue/treenode';
import type { TableColumn } from '../../types';
import { queryDataset } from '../../api';

import LeftNav from './LeftNav.vue';

import DataTable from 'primevue/datatable';
import Column from 'primevue/column';
import Textarea from 'primevue/textarea';
import Button from 'primevue/button';

import { IconCaretRightFilled, IconCaretDownFilled, IconSearch, IconRefresh } from '@tabler/icons-vue';

const nodes = ref([] as TreeNode[]);
const selectedDataset = ref({
  name: '',
  tableName: '',
  fields: [],
} as DatasetInfo);

const showFields = ref(false);
const query = ref('');
const isQuerying = ref(false);
const queryRsult = ref([]);
const queryColumns = ref([] as TableColumn[]);
const queryStatus = ref('');

onMounted(async () => {
  let items = await db.datasets.toArray();
  if (items.length === 0) {
    items = await loadDatasets();
  }
  selectedDataset.value = (await getCurrentDataset()) || items[0];
  console.log('selected:', selectedDataset.value);

  nodes.value = items.map((item) => {
    const node = {
      key: item.name,
      label: item.tableName,
      data: item,

      children: item.fields.map((field) => ({
        key: field.name,
        label: field.name.trim().toLowerCase(),
        selectable: false,
        leaf: true,
      })),
    };
    return node;
  });
});

const executeQuery = async () => {
  const start = Date.now();
  console.log('query:', query.value);
  try {
    isQuerying.value = true;
    queryStatus.value = 'Querying...';
    let data = await queryDataset(query.value, selectedDataset.value);
    queryStatus.value = `Query returned ${data.length} rows in ${Date.now() - start}ms`;
    isQuerying.value = false;
    nextTick(() => {
      let hasId = false;
      queryColumns.value = Object.keys(data[0]).map((key) => {
        if (key === 'id') {
          hasId = true;
        }
        return { name: key, label: key };
      });
      if (!hasId) {
        queryColumns.value.unshift({ name: 'id', label: 'id' });
        // add id to data
        data = data.map((item, index) => {
          item.id = index;
          return item;
        });
      }
      queryRsult.value = data;
    });

    setTimeout(() => {
      queryStatus.value = '';
    }, 5000);
    console.log('columns:', queryColumns.value);
  } catch (err) {
    queryStatus.value = err.message;
    isQuerying.value = false;
  }
};
</script>
