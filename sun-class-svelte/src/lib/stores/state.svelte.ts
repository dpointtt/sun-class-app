import { writable } from "svelte/store";

export const navbarState = $state({
	currentPage: "classes",
	className: "",
	classPath: "/",
	assignmentName: "",
});

export const classTab = writable<"enrolled" | "teaching">("enrolled")