<template>
  <aside class="flex overflow-y-auto bg-[#f8f9fa] w-64 shrink-0 xl:block border-r border-slate-300">
    <div class="flex p-2 bg-white shadow-lg">
      <h1 class="text-lg flex-1">Datasets</h1>
      <Button aria-label="Refresh" @click="refreshDataset">
        <IconRefresh :class="isRefreshing ? 'animate-spin' : 'animate-none'" />
      </Button>
    </div>
    <!-- Middle column area -->
    <div class="flex flex-col flex-1 overflow-x-hidden overflow-y-auto">
      <Tree
        :value="nodes"
        selection-mode="single"
        selection-keys="name"
        :metaKeySelection="false"
        class="w-full md:w-30rem"
        :pt="{
          content: ({ props, state, context }) => ({
            class: props.node.children ? 'p-0 m-0 cursor-pointer' : 'p-0 m-0 text-xs text-slate-700',
          }),
        }"
        @nodeSelect="onNodeSelect"
      >
      </Tree>
    </div>
  </aside>
</template>

<script setup lang="ts">
import { loadDatasets, setCurrentDataset, db } from '../../lib';
import { onMounted, ref } from 'vue';

import type { PropType } from 'vue';
import type { TreeNode } from 'primevue/treenode';

import Tree from 'primevue/tree';
import Button from 'primevue/button';
import { IconRefresh } from '@tabler/icons-vue';

defineProps({
  nodes: {
    type: Array as PropType<TreeNode[]>,
    required: true,
  },
});
const selectedDataset = defineModel('dataset');
const isRefreshing = ref(false);

onMounted(() => {});

const onNodeSelect = (node: TreeNode) => {
  setCurrentDataset(node.key);
  console.log('selected:', node.data);
  selectedDataset.value = node.data;
};

const refreshDataset = async () => {
  console.log('refresh dataset');
  isRefreshing.value = true;
  db.datasets.clear();
  await loadDatasets();
  isRefreshing.value = false;
};
</script>
