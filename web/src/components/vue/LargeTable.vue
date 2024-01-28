<template>
  <DynamicScroller
    class="border text-sm text-left rtl:text-right text-gray-500 dark:text-gray-400 h-96"
    :items="items"
    item-size="0"
    :min-item-size="50"
    :emit-update="true"
    @update="onUpdate"
  >
    <template #before>
      <div
        class="grid grid-flow-col justify-center items-center text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400"
      >
        <div class="px-6 py-3 w-64 border" v-for="col in columns" :key="col.name">
          {{ col.label }}
        </div>
      </div>
    </template>
    <template #default="{ item, index, active }">
      <DynamicScrollerItem
        :item="item"
        :active="active"
        :size-dependencies="[item]"
        :data-index="index"
        :data-active="active"
        class="w-full"
      >
        <div
          class="grid grid-flow-col auto-cols-auto items-center justify-center dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600"
        >
          <div class="p-1 m-0 w-64 border h-full" v-for="col in columns" :key="col.name">
            {{ item[col.name] || ' ' }}
          </div>
        </div>
      </DynamicScrollerItem>
    </template>
  </DynamicScroller>
</template>

<script setup lang="ts">
import { computed, type PropType } from 'vue';
import type { TableColumn } from '../../types';

const props = defineProps({
  columns: {
    type: Array as PropType<TableColumn[]>,
    required: true,
  },
  items: {
    type: Array,
    required: true,
  },
});

const updateParts = { viewStartIdx: 0, viewEndIdx: 0, visibleStartIdx: 0, visibleEndIdx: 0 };
const onUpdate = (viewStartIndex: number, viewEndIndex: number, visibleStartIndex: number, visibleEndIndex: number) => {
  console.log('onUpdate', viewStartIndex, viewEndIndex, visibleStartIndex, visibleEndIndex);
  updateParts.viewStartIdx = viewStartIndex;
  updateParts.viewEndIdx = viewEndIndex;
  updateParts.visibleStartIdx = visibleStartIndex;
  updateParts.visibleEndIdx = visibleEndIndex;
};
</script>
