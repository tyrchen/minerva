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
        :pt="{
          content: ({ props, state, context }) => ({
            class: props.node.children ? 'p-0 m-0' : 'p-0 m-0 text-xs text-slate-700',
          }),
        }"
        @nodeSelect="onNodeSelect"
      >
      </Tree>
    </div>
  </aside>
</template>

<script setup lang="ts">
import Tree from 'primevue/tree';
import { setCurrentDataset } from '../../api';
import { onMounted, ref, defineProps } from 'vue';
import type { TreeNode } from 'primevue/treenode';
import type { PropType } from 'vue';

defineProps({
  nodes: {
    type: Array as PropType<TreeNode[]>,
    required: true,
  },
});
const selectedDataset = defineModel('dataset');

onMounted(() => {});

const onNodeSelect = (node: TreeNode) => {
  setCurrentDataset(node.key);
  console.log('selected:', node.data);
  selectedDataset.value = node.data;
};
</script>
