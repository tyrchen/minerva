<template>
  <div @click="showFields = !showFields" class="cursor-pointer">
    <div class="my-4 flex items-center">
      <IconCaretRightFilled v-if="!showFields" />
      <IconCaretDownFilled v-if="showFields" />
      <h1 class="text-2xml">{{ name }}</h1>
      <span class="inline-block mx-4 text-sm text-gray-500">(last modified: {{ lastModified }})</span>
    </div>

    <DataTable
      class="my-4 max-w-7xl overflow-x-scroll"
      :value="fields"
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
    <DataTable :value="queryRsult" scrollable scrollHeight="600px" showGridlines tableStyle="min-width: 50rem">
      <Column sortable v-for="name in queryColumns" :field="name" :header="name"></Column>
    </DataTable>
  </div>
</template>

<script setup lang="ts">
import Textarea from 'primevue/textarea';
import Button from 'primevue/button';
import { IconCaretRightFilled, IconCaretDownFilled } from '@tabler/icons-vue';

import { ref, onMounted } from 'vue';
import { getCurrentDataset, queryDataset } from '../../api';
import DataTable from 'primevue/datatable';
import Column from 'primevue/column';
import type { DatasetField } from '@minerva/dataset-client';

const name = ref('');
const lastModified = ref();
const query = ref('');
const fields = ref([] as DatasetField[]);
const showFields = ref(false);
const queryRsult = ref([]);
const queryColumns = ref([]);

onMounted(async () => {
  const dataset = await getCurrentDataset();
  fields.value = dataset.fields;
  name.value = dataset.tableName;
  lastModified.value = dataset.lastModified;
});

const executeQuery = async () => {
  console.log('query:', query.value);
  let data = await queryDataset(query.value);
  queryRsult.value = data;
  queryColumns.value = Object.keys(data[0]).map((key) => key);
  console.log('columns:', queryColumns.value);
};
</script>
