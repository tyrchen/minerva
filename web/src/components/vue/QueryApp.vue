<template>
  <div class="flex flex-col flex-1">
    <div class="flex flex-1 overflow-y-auto">
      <!-- left column area -->
      <LeftNav :nodes="nodes" v-model:dataset="selectedDataset" />
      <main id="main" class="flex flex-1 overflow-y-auto">
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
          <div class="flex flex-col gap-4">
            <Textarea v-model="query" class="w-full text-xl" :rows="10" autoResize :disabled="isQuerying"> </Textarea>
            <div>
              <Button class="w-48" label="Execute!" raised size="large" @click="executeQuery">
                <IconSearch v-if="!isQuerying" />
                <IconRefresh v-if="isQuerying" class="animate-spin" />
                <span class="px-3">Execute!</span>
              </Button>
            </div>
            <span
              class="inline-block mx-4 text-sm"
              v-text="queryStatus"
              :class="isError ? 'text-red-500' : 'text-gray-500'"
            ></span>
          </div>

          <TabView>
            <TabPanel header="Output">
              <DataTable
                v-show="queryRsult.length > 0"
                class="my-4 overflow-x-scroll border"
                :style="{ width: mainWidth + 'px' }"
                paginator
                :rows="50"
                :rowsPerPageOptions="[50, 100, 200]"
                :value="queryRsult"
                size="small"
                stripedRows
              >
                <Column class="min-w-64" v-for="col in queryColumns" :field="col.name" :header="col.label"></Column>
              </DataTable>
            </TabPanel>
            <TabPanel header="Statistics">
              <div class="grid grid-cols-3 gap-4">
                <StatChart
                  v-for="name in Object.keys(queryColResult)"
                  :key="name"
                  :name="name"
                  :items="queryColResult[name]"
                />
              </div>
            </TabPanel>
          </TabView>
        </div>
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, nextTick, onMounted, ref } from 'vue';
import { queryDataset, tableToJson, tableToColumns, getCurrentDataset, loadDatasets, db } from '../../lib';

import type { DatasetInfo } from '@minerva/dataset-client';
import type { TreeNode } from 'primevue/treenode';
import type { TableColumn } from '../../types';

import LeftNav from './LeftNav.vue';
import StatChart from './StatChart.vue';

import DataTable from 'primevue/datatable';
import Column from 'primevue/column';
import Textarea from 'primevue/textarea';
import Button from 'primevue/button';
import TabView from 'primevue/tabview';
import TabPanel from 'primevue/tabpanel';

import { IconCaretRightFilled, IconCaretDownFilled, IconSearch, IconRefresh } from '@tabler/icons-vue';

const nodes = ref([] as TreeNode[]);
const selectedDataset = ref({} as DatasetInfo);

const showFields = ref(false);
const query = ref('');
const isQuerying = ref(false);
const queryRsult = ref([] as Object[]);
const queryColResult = ref({});
const queryColumns = ref([] as TableColumn[]);
const queryStatus = ref('');
const isError = ref(false);

const mainWidth = ref(0);
const getMainWidth = () => window.innerWidth - 256 - 144;

onMounted(async () => {
  mainWidth.value = getMainWidth();
  window.onresize = () => {
    mainWidth.value = getMainWidth();
    console.log('mainWidth:', mainWidth.value);
  };
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

      children: item.fields?.map((field) => ({
        key: field.name,
        label: field.name?.trim().toLowerCase(),
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
    isError.value = false;
    let table = await queryDataset(query.value, selectedDataset.value);
    let data = tableToJson(table);

    let columnItems = table.getChild('channelId');
    console.log(columnItems);

    queryColResult.value = tableToColumns(table, Object.keys(data[0]));

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
          item['id'] = index;
          return item;
        });
      }
      queryRsult.value = data;
    });
  } catch (err) {
    queryStatus.value = err.message;
    isError.value = true;
    isQuerying.value = false;
  }
};
</script>
