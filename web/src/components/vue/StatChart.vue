<template>
  <Card :pt="{ body: { class: 'py-2' }, content: { class: ' border-round-lg p-2' } }">
    <template #title>{{ name }}</template>
    <template #content>
      <Bar id="my-chart-id" :options="chartOptions" :data="chartData" />
      <span class="my-2 ml-2 text-xs">sdev: {{ jStat.stdev(items).toFixed(2) }}</span>
      <span class="my-2 ml-2 text-xs">mean: {{ jStat.mean(items).toFixed(2) }}</span>
      <span class="my-2 ml-2 text-xs">skewness: {{ jStat.skewness(items).toFixed(2) }}</span>
      <span class="my-2 ml-2 text-xs">coeffvar: {{ jStat.coeffvar(items).toFixed(2) }}</span>
      <span class="my-2 ml-2 text-xs">coeffvar: {{ jStat.coeffvar(items).toFixed(2) }}</span>
    </template>
  </Card>
</template>

<script setup lang="ts">
import jStat from 'jstat';
import { ref, onMounted } from 'vue';

import { Bar } from 'vue-chartjs';
import { Chart as ChartJS, Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale } from 'chart.js';
import Card from 'primevue/card';

ChartJS.register(Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale);

const props = defineProps({
  name: {
    type: String,
    required: true,
  },
  items: {
    type: Array,
    required: true,
  },
});

console.log('items:', props.items);

const chartData = {
  labels: props.items,
  datasets: [{ label: props.name, backgroundColor: '#34D399', data: props.items }],
};

const chartOptions = {
  responsive: true,
  animation: false,
  animations: {
    colors: false,
    x: false,
  },
  transitions: {
    active: {
      animation: {
        duration: 0,
      },
    },
  },
  plugins: {
    legend: {
      display: false,
    },
  },
  scales: {
    x: {
      display: false,
      title: {
        display: false,
      },
      grid: {
        display: false,
      },
    },
    y: {
      grid: {
        display: false,
      },
    },
  },
};

console.log('sdev:', jStat.stdev([1, 2, 3, 4, 5]));
</script>
