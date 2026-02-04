import { MaterialsSection } from "@/common/organisms";
import { MOCK_MATERIALS } from "@/mocks";

const MaterialsPage = () => {
  return (
    <div className="w-full xl:w-11/12 mx-auto max-w-screen-2xl px-4 py-6">
      <MaterialsSection materials={MOCK_MATERIALS} />
    </div>
  );
};

export default MaterialsPage;
