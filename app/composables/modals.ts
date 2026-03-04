import {
  MNotFound, MVscodeNotInstalled, MFolderHidden, MClearData,
} from '#components';
import { useModal } from 'vue-final-modal';
import type { Project } from '~/types/project';

export function useModals() {
  async function notFound(folder: string) {
    const { open, close } = useModal({
      component: MNotFound,
      attrs: {
        folder,
        async onClose(): Promise<void> {
          await close();
        },
      },
    });
    await open();
  }

  async function vsCodeNotInstalled() {
    const { open, close } = useModal({
      component: MVscodeNotInstalled,
      attrs: {
        async onClose(): Promise<void> {
          await close();
        },
      },
    });
    await open();
  }

  async function folderHidden(project: Project) {
    const { open, close } = useModal({
      component: MFolderHidden,
      attrs: {
        project,
        async onClose(): Promise<void> {
          await close();
        },
      },
    });
    await open();
  }

  async function clearData() {
    const { open, close } = useModal({
      component: MClearData,
      attrs: {
        async onClose(): Promise<void> {
          await close();
        },
      },
    });
    await open();
  }

  return {
    notFound,
    vsCodeNotInstalled,
    folderHidden,
    clearData,
  };
}
