<template>
  <aside class="flex overflow-y-auto bg-[#f8f9fa] w-64 shrink-0 xl:block border-r border-slate-300">
    <!-- Middle column area -->
    <div class="flex flex-col flex-1 overflow-x-hidden overflow-y-auto">
      <Tree
        :value="nodes"
        selection-mode="single"
        selection-keys="name"
        :metaKeySelection="false"
        class="w-full md:w-30rem"
        @nodeSelect="onNodeSelect"
      />
    </div>
  </aside>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { db } from '../../db';
import { loadDatasets, setCurrentDataset } from '../../api';
import Tree from 'primevue/tree';
import type { TreeNode } from 'primevue/treenode';

const nodes = ref([] as TreeNode[]);

onMounted(async () => {
  let items = await db.datasets.toArray();
  if (items.length === 0) {
    items = await loadDatasets();
  }

  const ret = items.map((item) => {
    const node = {
      key: item.name,
      label: item.tableName,
      children: item.fields.map((field) => ({
        key: field.name,
        label: `${field.name.trim().toLowerCase()} (${field.type})`,
        selectable: false,
      })),
    };
    return node;
  });

  nodes.value = ret;
});

const onNodeSelect = (node: TreeNode) => {
  setCurrentDataset(node.key);
  console.log('selected:', node);
};
</script>
