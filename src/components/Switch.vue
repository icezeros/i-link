<template>
  <div class="switch" @click="toggle">
    <div :class="{ knob: true, active: value }"></div>
  </div>
</template>

<script>
export default {
  data() {
    return {
      value: false,
    };
  },
  methods: {
    toggle() {
      this.value = !this.value;
    },
  },
};
</script>

<style scoped>
.switch {
  display: inline-block;
  width: 50px;
  height: 30px;
  background-color: #ddd;
  border-radius: 15px;
  position: relative;
  cursor: pointer;
}

.knob {
  position: absolute;
  top: 3px;
  left: 3px;
  width: 24px;
  height: 24px;
  border-radius: 50%;
  background-color: #fff;
  transition: all 0.2s ease-in-out;
}

.active {
  background-color: #66bb6a; /* switch打开时的背景色 */
  transform: translateX(20px);
}
</style>
