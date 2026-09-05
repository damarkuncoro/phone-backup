import { useState, useEffect } from 'react';
import { useDevices } from '@/features/devices/hooks/useDevices';
import { type Device } from '@/services/deviceService';
import { useBackupWizard } from '../hooks/useBackupWizard';
import { UI_TOKENS } from '@/shared/theme/tokens';
import { WizardHeader } from './WizardHeader';
import { WizardDeviceStep } from './WizardDeviceStep';
import { WizardDataStep } from './WizardDataStep';
import { DATA_OPTIONS } from '../lib/wizardDataOptions';
import { WizardReviewStep } from './WizardReviewStep';
import { WizardProgressStep } from './WizardProgressStep';

interface BackupWizardProps {
  initialDevice?: Device | null;
  onFinish?: () => void;
}

export function BackupWizard({ initialDevice, onFinish }: BackupWizardProps) {
  const { devices, loading: devicesLoading } = useDevices();
  const {
    step, setStep,
    selectedDevice, setSelectedDevice,
    selectedData, setSelectedData, toggleData,
    scannedFiles, isCalculating, reviewSearch, setReviewSearch,
    selectedPaths, handleTogglePath, handleNextToConfigure, handleStartBackup, handleExpressBackup,
    analysisState,
    progressMsg, progressPercent, totalItems, currentItems, error,
    selectedFiles, totalBytes,
    liveContacts, selectedContactIds, toggleContactId, selectAllContacts, deselectAllContacts,
    loadingStructured
  } = useBackupWizard();

  const [encryptionEnabled] = useState(true);

  // Auto-select initial device if passed from Dashboard
  useEffect(() => {
    if (initialDevice && !selectedDevice) {
      setSelectedDevice(initialDevice);
    }
  }, [initialDevice, selectedDevice, setSelectedDevice]);

  const isMtpDevice = selectedDevice?.connection_type === 'Mtp';

  return (
    <div className={UI_TOKENS.layout.pageContainer}>
      
      {/* Wizard Header Banner with Stepper */}
      <WizardHeader
        step={step}
        setStep={setStep}
        selectedDevice={selectedDevice}
        selectedDataCount={selectedData.length}
        progressPercent={progressPercent}
      />

      {/* Wizard Card Container */}
      <div className="bg-white rounded-[32px] border border-slate-100 shadow-sm flex flex-col overflow-hidden min-h-[520px]">
        
        {/* ================= STEP 1: SELECT DEVICE ================= */}
        {step === 'select-device' && (
          <WizardDeviceStep
            devices={devices}
            devicesLoading={devicesLoading}
            selectedDevice={selectedDevice}
            onSelectDevice={setSelectedDevice}
            onNext={() => setStep('select-data')}
          />
        )}

        {/* ================= STEP 2: SELECT DATA ================= */}
        {step === 'select-data' && (
          <WizardDataStep
            isMtpDevice={isMtpDevice}
            selectedData={selectedData}
            onToggleData={toggleData}
            onSelectAll={() => setSelectedData(DATA_OPTIONS.filter(o => !isMtpDevice || !o.requiresAdb).map(o => o.id))}
            onSelectMediaOnly={() => setSelectedData(['photos', 'files'])}
            onBack={() => setStep('select-device')}
            onExpressBackup={handleExpressBackup}
            onNext={handleNextToConfigure}
          />
        )}

        {/* ================= STEP 3: CONFIGURE & REVIEW ================= */}
        {step === 'configure' && (
          <WizardReviewStep
            totalBytes={totalBytes}
            selectedFilesCount={selectedFiles.length}
            reviewSearch={reviewSearch}
            onReviewSearchChange={setReviewSearch}
            isCalculating={isCalculating}
            analysisState={analysisState}
            scannedFiles={scannedFiles}
            selectedPaths={selectedPaths}
            onTogglePath={handleTogglePath}
            encryptionEnabled={encryptionEnabled}
            onBack={() => setStep('select-data')}
            onExpressBackup={handleExpressBackup}
            onStartBackup={() => handleStartBackup(DATA_OPTIONS.length)}
            selectedData={selectedData}
            liveContacts={liveContacts}
            selectedContactIds={selectedContactIds}
            onToggleContact={toggleContactId}
            onSelectAllContacts={selectAllContacts}
            onDeselectAllContacts={deselectAllContacts}
            loadingStructured={loadingStructured}
          />
        )}

        {/* ================= STEP 4: LIVE BACKUP STREAM ================= */}
        {step === 'progress' && (
          <WizardProgressStep
            error={error}
            progressPercent={progressPercent}
            progressMsg={progressMsg}
            totalItems={totalItems}
            currentItems={currentItems}
            onRetry={() => setStep('configure')}
            onFinish={() => onFinish ? onFinish() : window.location.reload()}
          />
        )}

      </div>
    </div>
  );
}
