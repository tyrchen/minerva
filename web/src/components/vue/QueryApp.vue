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
          <Textarea v-model="query" class="w-full h-48"></Textarea>
          <Button class="w-48 my-4" icon="pi pi-search" label="Execute!" raised size="large" @click="executeQuery" />

          <div class="max-w-7xl overflow-x-scroll my-4">
            <DataTable
              :value="queryRsult"
              size="small"
              scrollable
              scrollHeight="600px"
              showGridlines
              tableStyle="min-width: 50rem"
            >
              <Column v-for="name in queryColumns" :field="name" :header="name"></Column>
            </DataTable>
          </div>
        </div>
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import LeftNav from './LeftNav.vue';
import { db } from '../../db';
import { loadDatasets } from '../../api';
import type { DatasetInfo } from '@minerva/dataset-client';
import type { TreeNode } from 'primevue/treenode';

import { queryDataset } from '../../api';
import DataTable from 'primevue/datatable';
import Column from 'primevue/column';
import Textarea from 'primevue/textarea';
import Button from 'primevue/button';
import { IconCaretRightFilled, IconCaretDownFilled } from '@tabler/icons-vue';

const nodes = ref([] as TreeNode[]);
const selectedDataset = ref({
  name: '',
  tableName: '',
  fields: [],
} as DatasetInfo);

const showFields = ref(false);
const query = ref('');
const queryRsult = ref([]);
const queryColumns = ref([]);

onMounted(async () => {
  let items = await db.datasets.toArray();
  if (items.length === 0) {
    items = await loadDatasets();
  }
  selectedDataset.value = items[0];
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
  console.log('query:', query.value);
  let data = await queryDataset(query.value);
  queryRsult.value = data;
  queryColumns.value = Object.keys(data[0]).map((key) => key);
  console.log('columns:', queryColumns.value);
};
</script>
