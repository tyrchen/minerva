<template>
  <div>
    <div class="my-4 flex items-center cursor-pointer" @click="showFields = !showFields">
      <IconCaretRightFilled v-if="!showFields" />
      <IconCaretDownFilled v-if="showFields" />
      <h1 class="text-2xml">{{ name }}</h1>
      <span class="inline-block mx-4 text-sm text-gray-500">(last modified: {{ lastModified }})</span>
    </div>

    <DataTable
      class="my-4 max-w-7xl overflow-x-scroll"
      :value="fields"
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
</template>

<script setup lang="ts">
import Textarea from 'primevue/textarea';
import Button from 'primevue/button';
import { IconCaretRightFilled, IconCaretDownFilled } from '@tabler/icons-vue';

import { ref, onMounted, defineProps, type PropType } from 'vue';
import { queryDataset } from '../../api';
import DataTable from 'primevue/datatable';
import Column from 'primevue/column';
import type { DatasetField, DatasetInfo } from '@minerva/dataset-client';

const props = defineProps({
  dataset: {
    type: Object as PropType<DatasetInfo>,
    required: true,
  },
});

const name = ref('');
const lastModified = ref();

const fields = ref([] as DatasetField[]);
const showFields = ref(false);
const query = ref('');
const queryRsult = ref([]);
const queryColumns = ref([]);

onMounted(() => {
  console.log('dataset:', props.dataset);
  fields.value = props.dataset.fields;
  name.value = props.dataset.tableName;
  lastModified.value = props.dataset.lastModified;
});

const executeQuery = async () => {
  console.log('query:', query.value);
  let data = await queryDataset(query.value);
  queryRsult.value = data;
  queryColumns.value = Object.keys(data[0]).map((key) => key);
  console.log('columns:', queryColumns.value);
};
</script>
