<script setup>
import {
  ref,
  onMounted,
  onUnmounted,
  watchEffect,
  watch,
  reactive,
  computed,
} from "vue";
import {
  requestAccessibilityPermission,
  checkAccessibilityPermission,
  requestScreenRecordingPermission,
  checkScreenRecordingPermission,
} from "tauri-plugin-macos-permissions-api";
import { useStateStore } from "@utils/state-store";
import { msgError, msgSuccess } from "@utils/msg";
import { useRouter } from "vue-router";
const router = useRouter();
const stateStore = useStateStore();
const allPermissionsGranted = computed(() => {
  return (
    permissions.accessibility.granted && permissions.screenRecording.granted
  );
});
const permissions = reactive({
  accessibility: {
    granted: false,
    requesting: false,
  },
  screenRecording: {
    granted: false,
    requesting: false,
  },
});
const requestScreenRecording = async () => {
  permissions.screenRecording.requesting = true;
  try {
    await requestScreenRecordingPermission();
    const poll = async () => {
      const granted = await checkScreenRecordingPermission();
      if (granted) {
        permissions.screenRecording.requesting = false;
        permissions.screenRecording.granted = true;
        stateStore.app.enable.hasScreenRecordingPermission = true;
        msgSuccess("Screen Recording Permission Granted!");
      } else {
        setTimeout(poll, 200);
      }
    };
    poll();
  } catch (error) {
    msgError(
      "Screen Recording Permission request failed. Please grant it manually in System Preferences."
    );
    permissions.screenRecording.requesting = false;
  }
};

const requestAccessibility = async () => {
  permissions.accessibility.requesting = true;
  try {
    await requestAccessibilityPermission();
    const poll = async () => {
      const granted = await checkAccessibilityPermission();
      if (granted) {
        permissions.accessibility.requesting = false;
        permissions.accessibility.granted = true;
        stateStore.app.enable.hasAccessibilityPermission = true;
        msgSuccess("Accessibility Permission Granted!");
      } else {
        setTimeout(poll, 200);
      }
    };
    poll();
  } catch (error) {
    msgError(
      "Accessibility Permission request failed. Please grant it manually in System Preferences."
    );
    permissions.accessibility.requesting = false;
  }
};

const goHome = async () => {
  router.push({
    path: "/app/home",
  });
};

const mockPermissionRequest = async (type) => {
  return new Promise((resolve, reject) => {
    setTimeout(() => {
      // 模拟 80% 的成功率
      if (Math.random() > 0.2) {
        resolve();
      } else {
        reject(new Error("Permission denied"));
      }
    }, 2000);
  });
};
onMounted(async () => {});
onUnmounted(async () => {});
</script>
<template>
  <el-container>
    <el-main>
      <div class="permission-container">
        <div class="header">
          <h1>Permission Manager</h1>
        </div>

        <div class="permissions-list">
          <!-- 辅助功能权限 -->
          <div
            class="permission-card"
            :class="{ granted: permissions.accessibility.granted }"
          >
            <div class="permission-header">
              <div class="permission-title">
                <span class="permission-name">Accessibility Permission</span>
              </div>
              <el-tag
                v-if="permissions.accessibility.granted"
                type="success"
                size="large"
              >
                Granted
              </el-tag>
              <el-tag v-else type="warning" size="large"> Not Granted </el-tag>
            </div>
            <div class="permission-description">
              This permission allows Kiwi to control your computer interface for
              automated actions such as clicking and typing.
            </div>
            <el-button
              v-if="!permissions.accessibility.granted"
              type="primary"
              @click="requestAccessibility"
              :loading="permissions.accessibility.requesting"
            >
              {{
                permissions.accessibility.requesting
                  ? "Requesting..."
                  : "Request Permission"
              }}
            </el-button>
          </div>

          <!-- 屏幕录制权限 -->
          <div
            class="permission-card"
            :class="{ granted: permissions.screenRecording.granted }"
          >
            <div class="permission-header">
              <div class="permission-title">
                <span class="permission-name">Screen Recording Permission</span>
              </div>
              <el-tag
                v-if="permissions.screenRecording.granted"
                type="success"
                size="large"
              >
                Granted
              </el-tag>
              <el-tag v-else type="warning" size="large"> Not Granted </el-tag>
            </div>
            <div class="permission-description">
              This permission allows Kiwi to record your screen for capturing
              and recognizing screen content.
            </div>
            <el-button
              v-if="!permissions.screenRecording.granted"
              type="primary"
              @click="requestScreenRecording"
              :loading="permissions.screenRecording.requesting"
            >
              {{
                permissions.screenRecording.requesting
                  ? "Requesting..."
                  : "Request Permission"
              }}
            </el-button>
          </div>
        </div>

        <div class="footer">
          <el-button
            v-if="allPermissionsGranted"
            type="success"
            size="large"
            @click="goHome"
          >
            Start
          </el-button>
        </div>
      </div>
    </el-main>
  </el-container>
</template>
<style scoped>
.el-main {
  align-items: center;
  justify-content: center;
  height: 100vh;
  overflow: hidden;
  .permission-container {
    padding: 40px;
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
  }

  .header {
    text-align: center;
    margin-bottom: 40px;
  }

  .header h1 {
    font-size: 28px;
    color: #2c3e50;
    margin: 0 0 10px 0;
    font-weight: 600;
  }

  .header p {
    color: #7f8c8d;
    font-size: 16px;
    margin: 0;
  }

  .permissions-list {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 24px;
  }

  .permission-card {
    background: white;
    border-radius: 16px;
    padding: 24px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.05);
    border: 2px solid transparent;
    transition: all 0.3s ease;
  }

  .permission-card:hover {
    transform: translateY(-2px);
    box-shadow: 0 8px 25px rgba(0, 0, 0, 0.1);
  }

  .permission-card.granted {
    border-color: #67c23a;
    background: linear-gradient(135deg, #f0f9ff 0%, #e0f2fe 100%);
  }

  .permission-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 16px;
  }

  .permission-title {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .permission-icon {
    width: 32px;
    height: 32px;
    border-radius: 8px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 16px;
  }

  .accessibility-icon {
    background: linear-gradient(135deg, #4facfe 0%, #00f2fe 100%);
    color: white;
  }

  .screen-recording-icon {
    background: linear-gradient(135deg, #fa709a 0%, #fee140 100%);
    color: white;
  }

  .permission-name {
    font-size: 18px;
    font-weight: 600;
    color: #2c3e50;
  }

  .permission-description {
    color: #64748b;
    font-size: 14px;
    line-height: 1.6;
    margin-bottom: 16px;
  }

  .footer {
    margin-top: 32px;
    text-align: center;
  }

  .success-message {
    background: linear-gradient(135deg, #52c41a 0%, #73d13d 100%);
    color: white;
    padding: 16px 24px;
    border-radius: 12px;
    margin-bottom: 24px;
    text-align: center;
    font-weight: 600;
  }

  .el-button {
    font-weight: 600;
    border-radius: 8px;
    padding: 12px 24px;
  }

  .el-button--primary {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    border: none;
  }

  .el-button--success {
    background: linear-gradient(135deg, #52c41a 0%, #73d13d 100%);
    border: none;
  }

  .el-tag {
    border-radius: 6px;
    font-weight: 600;
  }
}
</style>
