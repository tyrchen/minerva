<template>
  <Dropdown
    v-model="instruction"
    editable
    :options="items"
    optionLabel="purpose"
    optionValue="sql"
    placeholder="Select or input a query"
    class="w-2/3 md:w-14rem"
    @keydown.enter="onEnter"
    @change="onChange"
  />
</template>

<script setup lang="ts">
import { computed, onMounted, ref, type PropType } from 'vue';
import { datasetTableSql, SqlAssistant, SqlSuggestionAssistant } from '~/lib';
import type { DatasetInfo } from '@minerva/dataset-client';

import Dropdown from 'primevue/dropdown';

interface SqlInfo {
  purpose: string;
  sql: string;
}

const props = defineProps({
  dataset: {
    type: Object as PropType<DatasetInfo>,
    required: true,
  },
});

const sql = defineModel('query', {
  type: String,
  default: '',
});

const instruction = ref('');
const items = ref([] as SqlInfo[]);

const tableSql = computed(() => {
  if (!props.dataset || Object.keys(props.dataset).length === 0) {
    return '';
  }
  return datasetTableSql(props.dataset);
});

const onEnter = async (e) => {
  console.log('onEnter:', e);
  if (instruction.value.trim()) {
    // let ret = await sqlSuggestion(tableSql.value);
    let assistant = new SqlAssistant(tableSql.value, instruction.value);
    let ret = await assistant.getCompletion();

    console.log('sql:', ret);
    sql.value = ret['sql'];
  }
};

const onChange = (e) => {
  console.log('onChange:', e);
  sql.value = e.value;
};

onMounted(async () => {
  const assistant = new SqlSuggestionAssistant(tableSql.value);
  const ret = await assistant.getCompletion();
  console.log('items:', ret);
  items.value = ret['queries'] as SqlInfo[];
});
</script>
