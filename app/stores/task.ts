interface TaskState {
  visible: boolean;
  text: string;
  type: 'default' | 'loading';
}

type Payload = Partial<Omit<TaskState, 'visible'>>;

export const useTaskStore = defineStore('Task', () => {
  const state = ref<TaskState>({
    text: '',
    type: 'default',
    visible: false,
  });

  function show(payload: Payload) {
    state.value.visible = true;
    state.value.text = payload.text ?? '';
    state.value.type = payload.type ?? 'default';
  }

  function update(payload: Payload) {
    state.value.text = payload.text ?? state.value.text;
    state.value.type = payload.type ?? state.value.type;
  }

  function hide() {
    state.value.visible = false;
    state.value.text = '';
    state.value.type = 'default';
  }

  return {
    state,
    show,
    update,
    hide,
  };
});
