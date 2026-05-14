<?php

/**
 * @file plugins/ojs/sourceright/classes/SourcerightCliRunner.php
 *
 * Safe command builder for invoking the Sourceright CLI from OJS.
 */

class SourcerightCliRunner
{
    private $cliPath;
    private $timeoutSeconds;

    public function __construct($cliPath = 'sourceright', $timeoutSeconds = 30)
    {
        $this->cliPath = $cliPath ?: 'sourceright';
        $this->timeoutSeconds = max(1, (int) $timeoutSeconds);
    }

    public function screenOjsSubmission($workspaceRoot, $submissionId, $manuscriptLabel = 'manuscript')
    {
        $result = $this->run([
            $this->cliPath,
            'journal-screen',
            '--platform',
            'ojs',
            '--submission-id',
            (string) $submissionId,
            '--manuscript',
            (string) $manuscriptLabel,
            (string) $workspaceRoot,
        ]);

        $payload = $this->decodeJsonPayload($result['stdout']);

        return [
            'editor' => [
                'summary' => $payload['editorial_summary'] ?? [],
                'report' => $payload,
            ],
            'author' => [
                'checklist' => $payload['author_action_checklist'] ?? [],
            ],
            'raw' => $result,
        ];
    }

    public function previewExport($workspaceRoot, $format)
    {
        $result = $this->run([
            $this->cliPath,
            'export',
            '--preview',
            '--format',
            (string) $format,
            (string) $workspaceRoot,
        ]);

        return [
            'editor' => [
                'preview' => $this->decodeJsonPayload($result['stdout']),
            ],
            'author' => [],
            'raw' => $result,
        ];
    }

    private function run(array $argv)
    {
        $command = $this->escapeCommand($argv);
        $descriptorSpec = [
            0 => ['pipe', 'r'],
            1 => ['pipe', 'w'],
            2 => ['pipe', 'w'],
        ];

        $process = proc_open($command, $descriptorSpec, $pipes);
        if (!is_resource($process)) {
            throw new RuntimeException('Unable to start Sourceright CLI process.');
        }

        fclose($pipes[0]);
        stream_set_blocking($pipes[1], false);
        stream_set_blocking($pipes[2], false);

        $stdout = '';
        $stderr = '';
        $deadline = time() + $this->timeoutSeconds;

        while (true) {
            $stdout .= stream_get_contents($pipes[1]);
            $stderr .= stream_get_contents($pipes[2]);

            $status = proc_get_status($process);
            if (!$status['running']) {
                break;
            }

            if (time() >= $deadline) {
                proc_terminate($process);
                fclose($pipes[1]);
                fclose($pipes[2]);
                proc_close($process);
                throw new RuntimeException('Sourceright CLI timed out.');
            }

            usleep(100000);
        }

        $exitCode = isset($status['exitcode']) ? (int) $status['exitcode'] : 1;
        $stdout .= stream_get_contents($pipes[1]);
        $stderr .= stream_get_contents($pipes[2]);
        fclose($pipes[1]);
        fclose($pipes[2]);
        proc_close($process);
        if ($exitCode !== 0) {
            throw new RuntimeException('Sourceright CLI failed: ' . trim($stderr));
        }

        return [
            'exitCode' => $exitCode,
            'stdout' => $stdout,
            'stderr' => $stderr,
        ];
    }

    private function escapeCommand(array $argv)
    {
        return implode(' ', array_map('escapeshellarg', $argv));
    }

    private function decodeJsonPayload($stdout)
    {
        $payload = json_decode($stdout, true);
        if (!is_array($payload) || json_last_error() !== JSON_ERROR_NONE) {
            throw new RuntimeException('Sourceright CLI did not return valid JSON.');
        }

        return $payload;
    }
}
