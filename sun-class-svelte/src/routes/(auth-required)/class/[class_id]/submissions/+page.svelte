<script lang="ts">
  import { navbarState } from "$lib/stores/state.svelte";
  import type { SubmissionListItem } from "$lib/types";
  import { Book, User, RotateCcw } from "@lucide/svelte";
  import type { PageProps } from "./$types";
  
  let { data }: PageProps = $props();
  
  navbarState.className = "Back to class page";
  navbarState.currentPage = "submissions";
  navbarState.classPath = "/class/" + data.class_id;

  // Filter states
  let showGraded = $state(true);
  let showUngraded = $state(true);
  let showResubmission = $state(true); // New filter for resubmission allowed
  let selectedAssignment = $state<string>("all");

  // Format date helper
  function formatDate(dateStr: string | null) {
    if (!dateStr) return "Not submitted";
    const d = new Date(dateStr);
    return d.toLocaleDateString() + " " + d.toLocaleTimeString();
  }

  // Check submission status
  function getSubmissionStatus(submission: SubmissionListItem) {
    if (submission.submitted_at === null) {
      return "resubmission"; // Grade was canceled, can resubmit
    } else if (submission.grade !== null && submission.grade !== undefined) {
      return "graded";
    } else {
      return "pending"; // Submitted but not graded yet
    }
  }

  // Group submissions by assignment
  const groupedSubmissions = $derived(() => {
    const grouped = new Map<string, SubmissionListItem[]>();
    
    data.submissions.forEach(submission => {
      const assignmentTitle = submission.assignment_title;
      if (!grouped.has(assignmentTitle)) {
        grouped.set(assignmentTitle, []);
      }
      grouped.get(assignmentTitle)!.push(submission);
    });

    // Sort submissions within each group by submission date (newest first)
    // Handle null submitted_at by putting them at the top
    grouped.forEach(submissions => {
      submissions.sort((a, b) => {
        // If both are null, maintain original order
        if (a.submitted_at === null && b.submitted_at === null) return 0;
        // If a is null, it should come first (resubmission allowed)
        if (a.submitted_at === null) return -1;
        // If b is null, it should come first
        if (b.submitted_at === null) return 1;
        // Both have dates, sort by newest first
        return new Date(b.submitted_at).getTime() - new Date(a.submitted_at).getTime();
      });
    });

    return grouped;
  });

  // Get unique assignment titles for filter dropdown
  const assignmentTitles = $derived(() => {
    const titles = Array.from(groupedSubmissions().keys()).sort();
    return titles;
  });

  // Filter submissions based on current filters
  const filteredGroupedSubmissions = $derived(() => {
    const filtered = new Map<string, SubmissionListItem[]>();

    groupedSubmissions().forEach((submissions, assignmentTitle) => {
      // Filter by assignment if specific one is selected
      if (selectedAssignment !== "all" && assignmentTitle !== selectedAssignment) {
        return;
      }

      // Filter by status
      const filteredSubmissions = submissions.filter(submission => {
        const status = getSubmissionStatus(submission);
        
        switch (status) {
          case "graded":
            return showGraded;
          case "pending":
            return showUngraded;
          case "resubmission":
            return showResubmission;
          default:
            return false;
        }
      });

      if (filteredSubmissions.length > 0) {
        filtered.set(assignmentTitle, filteredSubmissions);
      }
    });

    return filtered;
  });

  // Statistics
  const stats = $derived(() => {
    const total = data.submissions.length;
    const graded = data.submissions.filter(s => s.grade !== null && s.grade !== undefined).length;
    const pending = data.submissions.filter(s => s.submitted_at !== null && (s.grade === null || s.grade === undefined)).length;
    const resubmission = data.submissions.filter(s => s.submitted_at === null).length;
    const assignments = groupedSubmissions().size;

    return { total, graded, pending, resubmission, assignments };
  });
</script>

<div class="w-full h-full flex justify-center p-6">
  <div class="bg-black/50 box-shadow-lg backdrop-blur-sm p-3 rounded-2xl w-full max-w-screen-xl">
    <div class="bg-white/70 rounded-2xl p-6">
      <div class="flex justify-between items-center mb-6">
        <h2 class="text-2xl font-semibold">Student Submissions</h2>
        
        <!-- Statistics -->
        <div class="flex gap-4 text-sm">
          <div class="bg-blue-100 px-3 py-1 rounded-full">
            <span class="font-semibold">{stats().total}</span> Total
          </div>
          <div class="bg-green-100 px-3 py-1 rounded-full">
            <span class="font-semibold">{stats().graded}</span> Graded
          </div>
          <div class="bg-yellow-100 px-3 py-1 rounded-full">
            <span class="font-semibold">{stats().pending}</span> Pending
          </div>
          <div class="bg-orange-100 px-3 py-1 rounded-full">
            <span class="font-semibold">{stats().resubmission}</span> Resubmission
          </div>
          <div class="bg-purple-100 px-3 py-1 rounded-full">
            <span class="font-semibold">{stats().assignments}</span> Assignments
          </div>
        </div>
      </div>

      <!-- Filters -->
      <div class="mb-6 p-4 bg-white/50 rounded-xl">
        <div class="flex flex-wrap gap-4 items-center">
          <!-- Assignment Filter -->
          <div class="flex items-center gap-2">
            <label for="assignment-filter" class="text-sm font-medium">Assignment:</label>
            <select 
              id="assignment-filter"
              bind:value={selectedAssignment}
              class="px-3 py-1 rounded-lg border border-gray-300 bg-white text-sm"
            >
              <option value="all">All Assignments</option>
              {#each assignmentTitles() as title}
                <option value={title}>{title}</option>
              {/each}
            </select>
          </div>

          <!-- Status Filters -->
          <div class="flex items-center gap-4">
            <label class="flex items-center gap-2 text-sm">
              <input 
                type="checkbox" 
                bind:checked={showGraded}
                class="rounded"
              />
              <span class="select-none">Show Graded</span>
            </label>
            
            <label class="flex items-center gap-2 text-sm">
              <input 
                type="checkbox" 
                bind:checked={showUngraded}
                class="rounded"
              />
              <span class="select-none">Show Pending</span>
            </label>

            <label class="flex items-center gap-2 text-sm">
              <input 
                type="checkbox" 
                bind:checked={showResubmission}
                class="rounded"
              />
              <span class="select-none">Show Resubmission</span>
            </label>
          </div>

          <!-- Clear Filters -->
          <button 
            onclick={() => {
              selectedAssignment = "all";
              showGraded = true;
              showUngraded = true;
              showResubmission = true;
            }}
            class="px-3 py-1 text-sm bg-white hover:bg-gray-300 rounded-lg transition-colors"
          >
            Clear Filters
          </button>
        </div>
      </div>

      <!-- Submissions Content -->
      {#if filteredGroupedSubmissions().size === 0}
        <div class="text-center py-12">
          <p class="text-gray-500 text-lg">No submissions match your current filters.</p>
          <p class="text-gray-400 text-sm mt-2">Try adjusting your filter settings above.</p>
        </div>
      {:else}
        <div class="space-y-6">
          {#each Array.from(filteredGroupedSubmissions().entries()) as [assignmentTitle, submissions]}
            {@render assignment_group(assignmentTitle, submissions)}
          {/each}
        </div>
      {/if}
    </div>
  </div>
</div>

{#snippet assignment_group(assignmentTitle: string, submissions: SubmissionListItem[])}
  <div class="border border-gray-200 rounded-xl overflow-hidden">
    <!-- Assignment Header -->
    <div class="bg-gray-50 px-4 py-3 border-b border-gray-200">
      <div class="flex justify-between items-center">
        <h3 class="font-semibold text-lg">{assignmentTitle}</h3>
        <div class="flex gap-2 text-sm">
          <span class="px-2 py-1 bg-blue-100 text-blue-700 rounded-full">
            {submissions.length} submission{submissions.length !== 1 ? 's' : ''}
          </span>
          {#if submissions.some(s => s.grade !== null && s.grade !== undefined)}
            <span class="px-2 py-1 bg-green-100 text-green-700 rounded-full">
              {submissions.filter(s => s.grade !== null && s.grade !== undefined).length} graded
            </span>
          {/if}
          {#if submissions.some(s => s.submitted_at === null)}
            <span class="px-2 py-1 bg-orange-100 text-orange-700 rounded-full">
              {submissions.filter(s => s.submitted_at === null).length} resubmission
            </span>
          {/if}
        </div>
      </div>
    </div>

    <!-- Submissions List -->
    <div class="divide-y divide-gray-100">
      {#each submissions as submission}
        <a 
          href={`/class/${data.class_id}/submissions/${submission.id}`} 
          class="block hover:bg-gray-50 transition-colors"
        >
          {@render submission_block(submission)}
        </a>
      {/each}
    </div>
  </div>
{/snippet}

{#snippet submission_block(submission: SubmissionListItem)}
  <div class="p-4 flex bg-white/50 justify-between items-center">
    <div class="flex items-center gap-4">
      {#if submission.submitted_at === null}
        <RotateCcw size="24" class="text-orange-600" />
      {:else}
        <Book size="24" />
      {/if}
      
      <div>
        <p class="font-semibold text-lg">{submission.student_name}</p>
        <p class="text-sm text-gray-600">
          {#if submission.submitted_at === null}
            Grade canceled - can resubmit
          {:else}
            Submitted {formatDate(submission.submitted_at)}
          {/if}
        </p>
      </div>
    </div>

    <div class="flex items-center gap-3">
      <!-- Grade Status -->
      {#if submission.submitted_at === null}
        <span class="px-3 py-1 text-orange-700 bg-orange-100 rounded-full text-sm font-semibold">
          Resubmission Allowed
        </span>
      {:else if submission.grade !== null && submission.grade !== undefined}
        <div class="text-right">
          <span class="px-3 py-1 text-green-700 bg-green-100 rounded-full text-sm font-semibold">
            Grade: {submission.grade}
          </span>
        </div>
      {:else}
        <span class="px-3 py-1 text-yellow-700 bg-yellow-100 rounded-full text-sm font-semibold">
          Pending Review
        </span>
      {/if}

      <!-- Arrow Icon -->
      <svg class="w-5 h-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path>
      </svg>
    </div>
  </div>
{/snippet}